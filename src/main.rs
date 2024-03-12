use std::net::TcpListener;
use std::sync::Arc;
use std::{thread, time::Duration};

use book_server::mk_server;
use book_server::response::basic_responses::internal_server_error;
use book_server::response::file_response::file_response;
use book_server::response::{RespStatus, Response, ResponseBuilder};
use book_server::route::RouteParams;
use book_server::server::run_server;

fn main() {
    fn get_root(_params: &RouteParams) -> Response {
        file_response("res/hello.html")
    }

    // For testing multithreading
    fn get_sleep(_params: &RouteParams) -> Response {
        thread::sleep(Duration::from_secs(4));
        file_response("res/hello.html")
    }

    fn get_howdy(params: &RouteParams) -> Response {
        match params.get("name") {
            // TODO this should never be reached
            // there should be some typesafe way to access uri matched properties
            None => internal_server_error(),
            Some(name) => ResponseBuilder::new(RespStatus::Ok)
                .content(format!("<h3>Nice to meet you, {name}!</h3>"))
                .build(),
        }
    }

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let server = Arc::new(mk_server!(
        "/" => get_root,
        "/sleep" => get_sleep,
        "/howdy/:name" => get_howdy,
    ));

    run_server(&listener, &server);
}
