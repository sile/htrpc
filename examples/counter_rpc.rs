extern crate clap;
extern crate fibers;
extern crate futures;
#[macro_use]
extern crate htrpc;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sloggers;
#[macro_use]
extern crate trackable;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use clap::{App, Arg, SubCommand};
use fibers::{Executor, Spawn, InPlaceExecutor};
use futures::{BoxFuture, Future};
use htrpc::{RpcClient, RpcServerBuilder};
use htrpc::{Procedure, RpcRequest, RpcResponse, HandleRpc};
use htrpc::types::{EntryPoint, HttpMethod, NeverFail};
use sloggers::Build;

fn main() {
    let matches = App::new("counter_rpc")
        .arg(Arg::with_name("HOST")
                 .short("h")
                 .long("host")
                 .takes_value(true)
                 .default_value("127.0.0.1"))
        .arg(Arg::with_name("PORT")
                 .short("p")
                 .long("port")
                 .takes_value(true)
                 .default_value("3000"))
        .subcommand(SubCommand::with_name("server"))
        .subcommand(SubCommand::with_name("client")
                        .arg(Arg::with_name("COUNTER_NAME")
                                 .long("counter")
                                 .takes_value(true)
                                 .default_value("foo"))
                        .arg(Arg::with_name("COUNT_VALUE")
                                 .short("n")
                                 .long("count_value")
                                 .takes_value(true)
                                 .default_value("1")))
        .get_matches();
    let host = matches.value_of("HOST").unwrap();
    let port = matches.value_of("PORT").unwrap();
    let server_addr = format!("{}:{}", host, port);
    let mut executor = InPlaceExecutor::new().unwrap();

    if let Some(matches) = matches.subcommand_matches("client") {
        let counter = matches.value_of("COUNTER_NAME").unwrap();
        let count_value = matches.value_of("COUNT_VALUE").unwrap();

        let mut client = RpcClient::new(server_addr.parse().unwrap());
        let request = FetchAndAddRequest {
            path: (counter.to_string(),),
            query: Value { value: count_value.parse().unwrap() },
        };
        let future = client.call::<FetchAndAdd>(request);

        let monitor = executor.spawn_monitor(future);
        let result = executor.run_fiber(monitor).unwrap();
        println!("RESULT: {:?}", result);
    } else if let Some(_matches) = matches.subcommand_matches("server") {
        let mut builder = RpcServerBuilder::new(server_addr.parse().unwrap());
        track_try_unwrap!(builder.register(FetchAndAddHandler::new()));
        let logger = track_try_unwrap!(sloggers::terminal::TerminalLoggerBuilder::new()
                                           .level(sloggers::types::Severity::Debug)
                                           .build());
        builder.set_logger(logger);
        let server = builder.start(executor.handle());
        let monitor = executor.spawn_monitor(server.map_err(|e| panic!("{:?}", e)));
        executor.run_fiber(monitor).unwrap().unwrap();
    } else {
        println!("{}", matches.usage());
    }
}

struct FetchAndAdd;
impl Procedure for FetchAndAdd {
    type Request = FetchAndAddRequest;
    type Response = FetchAndAddResponse;
    fn method() -> HttpMethod {
        HttpMethod::Put
    }
    fn entry_point() -> EntryPoint {
        htrpc_entry_point!["counters", _]
    }
}

#[derive(Clone)]
struct FetchAndAddHandler {
    counters: Arc<Mutex<HashMap<String, usize>>>,
}
impl FetchAndAddHandler {
    pub fn new() -> Self {
        FetchAndAddHandler { counters: Arc::new(Mutex::new(HashMap::new())) }
    }
}
impl HandleRpc<FetchAndAdd> for FetchAndAddHandler {
    type Future = BoxFuture<FetchAndAddResponse, NeverFail>;
    fn handle_rpc(self, request: FetchAndAddRequest) -> Self::Future {
        let FetchAndAddRequest {
            path: (name,),
            query: Value { value },
        } = request;
        let mut counters = self.counters.lock().expect("TODO");
        *counters.entry(name.clone()).or_insert(0) += value as usize;

        let value = *counters.get(&name).unwrap();
        futures::finished(FetchAndAddResponse::Ok { body: Value { value } }).boxed()
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum FetchAndAddResponse {
    Ok {
        #[serde(with = "htrpc::json")]
        body: Value,
    },
}
impl RpcResponse for FetchAndAddResponse {}

#[derive(Debug, Serialize, Deserialize)]
struct FetchAndAddRequest {
    pub path: (String,),
    pub query: Value,
}
impl RpcRequest for FetchAndAddRequest {}

#[derive(Debug, Serialize, Deserialize)]
struct Value {
    #[serde(default = "one")]
    pub value: usize,
}
fn one() -> usize {
    1
}
