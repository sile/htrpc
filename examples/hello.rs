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

use clap::{App, Arg, SubCommand};
use fibers::{Executor, Spawn, ThreadPoolExecutor};
use futures::{BoxFuture, Future};
use htrpc::{RpcClient, RpcServerBuilder};
use htrpc::{Procedure, RpcRequest, RpcResponse, HandleRpc};
use htrpc::types::{EntryPoint, HttpMethod, NeverFail};
use sloggers::Build;

fn main() {
    let matches = App::new("hello")
        .arg(
            Arg::with_name("HOST")
                .short("h")
                .long("host")
                .takes_value(true)
                .default_value("127.0.0.1"),
        )
        .arg(
            Arg::with_name("PORT")
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value("8080"),
        )
        .subcommand(SubCommand::with_name("server"))
        .subcommand(SubCommand::with_name("client"))
        .get_matches();
    let host = matches.value_of("HOST").unwrap();
    let port = matches.value_of("PORT").unwrap();
    let server_addr = format!("{}:{}", host, port);
    let mut executor = ThreadPoolExecutor::new().unwrap();

    if let Some(_matches) = matches.subcommand_matches("client") {
        let mut client = RpcClient::new(server_addr.parse().unwrap());
        let request = HelloRequest {};
        let future = client.call::<Hello>(request);

        let monitor = executor.spawn_monitor(future);
        let result = executor.run_fiber(monitor).unwrap();
        println!("RESULT: {:?}", result);
    } else if let Some(_matches) = matches.subcommand_matches("server") {
        let mut builder = RpcServerBuilder::new(server_addr.parse().unwrap());
        track_try_unwrap!(builder.register(HelloHandler));
        let logger = track_try_unwrap!(
            sloggers::terminal::TerminalLoggerBuilder::new()
                .level(sloggers::types::Severity::Debug)
                .build()
        );
        builder.set_logger(logger);
        let server = builder.start(executor.handle());
        let monitor = executor.spawn_monitor(server.map_err(|e| panic!("{:?}", e)));
        executor.run_fiber(monitor).unwrap().unwrap();
    } else {
        println!("{}", matches.usage());
    }
}

struct Hello;
impl Procedure for Hello {
    type Request = HelloRequest;
    type Response = HelloResponse;
    fn method() -> HttpMethod {
        HttpMethod::Get
    }
    fn entry_point() -> EntryPoint {
        htrpc_entry_point!["hello"]
    }
}

#[derive(Clone)]
struct HelloHandler;
impl HandleRpc<Hello> for HelloHandler {
    type Future = BoxFuture<HelloResponse, NeverFail>;
    fn handle_rpc(self, request: HelloRequest) -> Self::Future {
        let HelloRequest {} = request;
        futures::finished(HelloResponse::Ok { body: HelloWorld::HelloWorld }).boxed()
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum HelloResponse {
    Ok { body: HelloWorld },
}
impl RpcResponse for HelloResponse {}

#[derive(Debug, Serialize, Deserialize)]
struct HelloRequest {}
impl RpcRequest for HelloRequest {}

#[derive(Debug, Serialize, Deserialize)]
enum HelloWorld {
    HelloWorld,
}
