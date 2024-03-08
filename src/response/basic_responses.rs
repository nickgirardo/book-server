use super::{RespStatus, Response, ResponseBuilder};

// 400 BAD REQUEST
pub fn bad_request() -> Response {
    ResponseBuilder::new(RespStatus::HttpVersionNotSupported)
        .content(String::from("HTTP Version Not Supported"))
        .build()
}

// 404 NOT FOUND
pub fn not_found() -> Response {
    ResponseBuilder::new(RespStatus::NotFound)
        .content(String::from("Not found"))
        .build()
}

// 405 METHOD NOT ALLOWED
pub fn method_not_allowed() -> Response {
    ResponseBuilder::new(RespStatus::MethodNotAllowed)
        .content(String::from("Method Not Allowed"))
        .build()
}

// 500 INTERNAL SERVER ERROR
pub fn internal_server_error() -> Response {
    ResponseBuilder::new(RespStatus::InternalServerError)
        .content(String::from("Internal Server Error"))
        .build()
}

// 505 HTTP VERSION NOT SUPPORTED
pub fn http_version_not_supported() -> Response {
    ResponseBuilder::new(RespStatus::HttpVersionNotSupported)
        .content(String::from("HTTP Version Not Supported"))
        .build()
}
