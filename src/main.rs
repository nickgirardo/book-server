use std::net::TcpListener;
use std::sync::Arc;
use std::{thread, time::Duration};

use book_server::mk_server;
use book_server::response::basic_responses::internal_server_error;
use book_server::response::file_response::file_response;
use book_server::response::{RespStatus, ResponseBuilder};
use book_server::route::RouteParams;
use book_server::server::run_server;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let server = Arc::new(mk_server!(
        "/" => |_params: &RouteParams| file_response("res/hello.html"),
        "/sleep" => |_params: &RouteParams| {
            thread::sleep(Duration::from_secs(4));
            file_response("res/hello.html")
        },
        "/howdy/:name" => |params: &RouteParams| {
            match params.get("name") {
                // TODO this should never be reached
                // there should be some typesafe way to access uri matched properties
                None => internal_server_error(),
                Some(name) => ResponseBuilder::new(RespStatus::Ok)
                    .content(format!("<h3>Nice to meet you, {name}!</h3>"))
                    .build(),
            }
        },
    ));

    run_server(&listener, &server);
}
