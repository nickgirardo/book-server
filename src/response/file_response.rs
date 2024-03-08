use super::basic_responses::not_found;
use super::{RespStatus, Response, ResponseBuilder};

use std::fs;

// TODO should this be an &str?  Pretty sure rust has a more specific type for paths
pub fn file_response(path: &str) -> Response {
    if let Ok(content) = fs::read_to_string(path) {
        ResponseBuilder::new(RespStatus::Ok)
            .content(content)
            .build()
    } else {
        not_found()
    }
}
