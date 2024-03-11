pub type Request = (ReqMethod, String, ReqHttpVersion);

#[derive(Debug, PartialEq, Eq)]
pub enum ReqMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
    Other(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReqHttpVersion {
    Http1_1,
    NotHttp1_1,
}

pub fn parse_request_line(request_line: &str) -> Option<Request> {
    let request_parts: Vec<_> = request_line.split(' ').collect();

    if let [method, uri, version] = request_parts[..] {
        let version = if version == "HTTP/1.1" {
            ReqHttpVersion::Http1_1
        } else {
            ReqHttpVersion::NotHttp1_1
        };

        let method = match method {
            "GET" => ReqMethod::Get,
            "HEAD" => ReqMethod::Head,
            "POST" => ReqMethod::Post,
            "PUT" => ReqMethod::Put,
            "DELETE" => ReqMethod::Delete,
            "CONNECT" => ReqMethod::Connect,
            "OPTIONS" => ReqMethod::Options,
            "TRACE" => ReqMethod::Trace,
            "PATCH" => ReqMethod::Patch,
            other => ReqMethod::Other(String::from(other)),
        };

        Some((method, String::from(uri), version))
    } else {
        None
    }
}
