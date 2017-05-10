extern crate clap;
extern crate fibers;
extern crate futures;
extern crate htrpc;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use clap::{App, Arg};
use fibers::{Executor, Spawn, InPlaceExecutor};
use futures::BoxFuture;
use htrpc::Method;
use htrpc::client::RpcClient;
use htrpc::procedure::{Procedure, RpcInput, RpcInputBody, RpcOutput, ContentType, Unreachable,
                       EntryPoint};
use htrpc::path_template::{PathTemplate, PathSegment};
use serde::{Serialize, Deserialize};

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
        .arg(Arg::with_name("COUNTER_NAME")
                 .long("counter")
                 .takes_value(true)
                 .default_value("foo"))
        .arg(Arg::with_name("COUNT_VALUE")
                 .short("n")
                 .long("count_value")
                 .takes_value(true)
                 .default_value("1"))
        .get_matches();
    let host = matches.value_of("HOST").unwrap();
    let port = matches.value_of("PORT").unwrap();
    let server = format!("{}:{}", host, port);
    let counter = matches.value_of("COUNTER_NAME").unwrap();
    let count_value = matches.value_of("COUNT_VALUE").unwrap();

    let mut executor = InPlaceExecutor::new().unwrap();
    let mut client = RpcClient::new(server.parse().unwrap());
    let input = FetchAndAddInput {
        counter: counter.to_string(),
        value: count_value.parse().unwrap(),
    };
    let future = client.call::<FetchAndAdd>(input);

    let monitor = executor.spawn_monitor(future);
    executor.run_fiber(monitor).unwrap().unwrap();
}

struct FetchAndAdd;
impl Procedure for FetchAndAdd {
    type Input = FetchAndAddInput;
    type Output = FetchAndAddOutput;

    fn entry_point() -> EntryPoint {
        use htrpc::path_template::PathSegment::*;
        static SEGMENTS: &[PathSegment] = &[Val("counters"), Var];
        EntryPoint {
            method: Method::Put,
            path: PathTemplate::new(SEGMENTS),
        }
    }

    fn handle_call(self, _input: Self::Input) -> BoxFuture<Self::Output, Unreachable> {
        panic!()
    }
}

struct FetchAndAddOutput;
impl RpcOutput for FetchAndAddOutput {}

struct FetchAndAddInput {
    pub counter: String,
    pub value: u8,
}
impl RpcInput for FetchAndAddInput {
    type Path = (String,);
    type Query = AddValue;
    type Header = EmptyHeader;
    type Body = EmptyBody;
    fn compose(path: Self::Path,
               query: Self::Query,
               _header: Self::Header,
               _body: Self::Body)
               -> htrpc::Result<Self> {
        Ok(FetchAndAddInput {
               counter: path.0,
               value: query.value,
           })
    }
    fn decompose(self) -> htrpc::Result<(Self::Path, Self::Query, Self::Header, Self::Body)> {
        let path = (self.counter,);
        let query = AddValue { value: self.value };
        let header = EmptyHeader {};
        let body = EmptyBody;
        Ok((path, query, header, body))
    }
}

#[derive(Serialize, Deserialize)]
struct AddValue {
    #[serde(default = "one")]
    pub value: u8,
}
fn one() -> u8 {
    1
}

#[derive(Serialize, Deserialize)]
struct EmptyHeader {}

#[derive(Serialize, Deserialize)]
struct EmptyBody;
impl RpcInputBody for EmptyBody {
    type ContentType = ContentTypeVoid;
}

struct ContentTypeVoid;
impl ContentType for ContentTypeVoid {
    fn mime() -> Option<&'static str> {
        None
    }
    fn serialize_body<T>(_body: T) -> htrpc::Result<Vec<u8>>
        where T: Serialize
    {
        Ok(Vec::new())
    }
    fn deserialize_body<T>(_bytes: Vec<u8>) -> htrpc::Result<T>
        where T: for<'a> Deserialize<'a>
    {
        panic!()
    }
}
