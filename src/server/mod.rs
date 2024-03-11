use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use crate::{request::parse_request_line, response::basic_responses::bad_request};

use super::request::{ReqHttpVersion, ReqMethod, Request};
use super::response::basic_responses::{http_version_not_supported, method_not_allowed, not_found};
use super::response::{strip_response_body, RespStatus, Response, ResponseBuilder};
use super::route::RouteHandler;
use super::thread_pool::ThreadPool;

// NOTE just made this macro for a bit of fun lol
#[macro_export]
macro_rules! mk_server {
    ($($route:literal => $handler:expr),+ $(,)?) => {
        $crate::server::Server::new(vec![
            $($crate::route::RouteHandler::new($crate::route::Route::new($route).unwrap(), $handler),)+
        ])
    };
}

pub struct Server {
    routes: Vec<RouteHandler>,
}

impl Server {
    pub fn new(routes: Vec<RouteHandler>) -> Self {
        Server { routes }
    }

    pub fn exec_routes(&self, method: ReqMethod, uri: &str) -> Response {
        let found = self
            .routes
            .iter()
            .find_map(|route_handler| route_handler.0.match_route(uri, route_handler));

        let (req_handler, route_params) = match found {
            None => return not_found(),
            Some(find) => find,
        };

        // NOTE currently all routes define a GET handler and no other handlers
        // In the future, when routes can define more handlers
        match method {
            ReqMethod::Get => req_handler.1(route_params),
            ReqMethod::Head => strip_response_body(req_handler.1(route_params)),
            // NOTE once again, this assumes we only support GETs which is currently true
            ReqMethod::Options => ResponseBuilder::new(RespStatus::Ok)
                .header(String::from("allow"), String::from("OPTIONS HEAD GET"))
                .build(),
            _ => method_not_allowed(),
        }
    }
}

fn handle_connection(mut stream: TcpStream, server_cfg: Arc<Server>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next();

    // TODO unsure why we're occasionally receiving empty requests, probably a TCP thing?
    if request_line.is_none() {
        println!("Empty request");
        return;
    }

    // TODO only parsing the first line of the request
    // this ignores headers and bodies
    let parsed_request = parse_request_line(&request_line.unwrap().unwrap());

    let resp: Response = if let Some(req) = parsed_request {
        server(server_cfg, req)
    } else {
        bad_request()
    };

    stream.write_all(resp.to_string().as_bytes()).unwrap();
}

fn server(server_cfg: Arc<Server>, req: Request) -> Response {
    let (method, uri, version) = req;

    match (method, uri.as_str(), version) {
        // Only supporting HTTP/1.1
        (_, _, ReqHttpVersion::NotHttp1_1) => http_version_not_supported(),
        // Wildcard OPTIONS request for whole server support
        // Only supporting OPTIONS, HEAD and GET requests right now
        (ReqMethod::Options, "*", _) => ResponseBuilder::new(RespStatus::NoContent)
            .header(String::from("allow"), String::from("OPTIONS HEAD GET"))
            .build(),
        (method, uri, _) => server_cfg.exec_routes(method, uri),
    }
}

pub fn run_server(listener: TcpListener, server_cfg: Arc<Server>) {
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let server = Arc::clone(&server_cfg);

        pool.execute(move || {
            handle_connection(stream, server);
        });
    }
}
