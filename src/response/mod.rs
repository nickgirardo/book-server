pub mod basic_responses;
pub mod file_response;

use std::fmt;

// NOTE adding response statuses as they're needed
#[derive(Clone, Copy, Debug)]
pub enum RespStatus {
    Ok,
    NoContent,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
    HttpVersionNotSupported,
}

impl RespStatus {
    fn as_str(self) -> &'static str {
        match self {
            RespStatus::Ok => "200 OK",
            RespStatus::NoContent => "204 NO CONTENT",
            RespStatus::BadRequest => "400 BAD REQUEST",
            RespStatus::NotFound => "404 NOT FOUND",
            RespStatus::MethodNotAllowed => "405 METHOD NOT ALLOWED",
            RespStatus::InternalServerError => "500 INTERNAL SERVER ERROR",
            RespStatus::HttpVersionNotSupported => "505 HTTP VERSION NOT SUPPORTED",
        }
    }
}

impl fmt::Display for RespStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// NOTE content should just be a byte array
// but for now only allowing utf-8 responses is fine enough
pub struct Response {
    status: RespStatus,
    headers: Vec<(String, String)>,
    content: String,
}

pub struct ResponseBuilder {
    status: RespStatus,
    headers: Vec<(String, String)>,
    content: String,
}

impl ResponseBuilder {
    pub fn new(status: RespStatus) -> Self {
        Self {
            status,
            headers: Vec::new(),
            content: String::new(),
        }
    }

    pub fn header(&mut self, name: String, value: String) -> &mut Self {
        self.headers.push((name, value));
        self
    }

    pub fn content(&mut self, content: String) -> &mut Self {
        self.content = content;
        self
    }

    pub fn build(&self) -> Response {
        Response {
            status: self.status,
            headers: self.headers.clone(),
            content: self.content.clone(),
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Status line
        let _ = write!(f, "HTTP/1.1 {}\r\n", self.status.as_str());

        // Normal headers
        for (name, value) in &self.headers {
            let _ = write!(f, "{name}: {value}\r\n");
        }

        // Content length as the final header, an extra CRLF, then the content itself
        let length = self.content.len();
        write!(f, "content-length: {length}\r\n\r\n{}", self.content)
    }
}

// Useful for HEAD requests, for which we want to perform a GET but return an empty body
pub fn strip_response_body(resp: Response) -> Response {
    Response {
        status: resp.status,
        headers: resp.headers,
        content: String::new(),
    }
}
