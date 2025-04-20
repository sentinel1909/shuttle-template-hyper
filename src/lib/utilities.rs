// src/lib/utilities.rs

// dependencies
use crate::types::JsonResponse;
use http_body_util::{
    Empty, Full,
    {BodyExt, combinators::BoxBody},
};
use hyper::body::Bytes;
use hyper::header::{CONTENT_TYPE, HeaderValue};
use hyper::{Error, Response};
use serde::Serialize;
use std::collections::HashMap;
use tokio::signal;

// utility function which provides a shutdown signal; leverage Tokio::signal
pub async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install CTRL-C signal handler");
}

// utility function to create an empty body for a Response
pub fn empty() -> BoxBody<Bytes, Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

pub fn set_content_type_json<T>(response: &mut Response<T>) {
    response
        .headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
}

// utility function to create a JSON response body
pub fn json_response_msg<T: Serialize>(value: T) -> BoxBody<Bytes, Error> {
    let wrapper = JsonResponse {
        msg: "success",
        content: value,
    };

    let json = serde_json::to_vec(&wrapper).unwrap_or_else(|e| {
        let fallback = format!(r#"{{"msg":"error","error":"{}"}}"#, e);
        fallback.into_bytes()
    });

    Full::new(Bytes::from(json))
        .map_err(|never| match never {})
        .boxed()
}

// utility to parse query parameters into individual pieces
// Parse "key=value&foo=bar" into HashMap<String, String>
pub fn parse_query_string(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter(|pair| !pair.is_empty())
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next().unwrap_or("");
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_query_returns_empty_map() {
        let result = parse_query_string("");
        assert!(result.is_empty());
    }
}
