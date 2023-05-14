use hyper::{header, Body, Request};

use crate::error::NoHostError;

pub fn get_host(req: &Request<Body>) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Attempt to get Host header
    let host = req.headers().get(header::HOST);

    // Get the Authority in the request start line
    let uri = req.uri().authority();

    // TODO: Appropriate logic based on https://tools.ietf.org/html/rfc7230#section-5.4
    match (host, uri) {
        (Some(host), None) => Ok(host.to_str()?.to_string()),
        (None, Some(authority)) => Ok(authority.as_str().to_string()),
        (Some(_), Some(authority)) => Ok(authority.as_str().to_string()),
        (None, None) => Err(Box::new(NoHostError)),
    }
}
