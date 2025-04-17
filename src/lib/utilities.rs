// src/lib/utilities.rs

// dependencies
use http_body_util::{
    Empty, Full,
    {combinators::BoxBody, BodyExt},
};
use hyper::body::Bytes;
use hyper::Error;
use serde::Serialize;
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

// utility function to create a response body with any desired message
pub fn response_msg<T: Into<Bytes>>(text: T) -> BoxBody<Bytes, Error> {
    Full::new(text.into())
        .map_err(|never| match never {})
        .boxed()
}

// utility function to create a JSON response body
pub fn json_response_msg<T: Serialize>(value: T) -> BoxBody<Bytes, Error> {
    let json = serde_json::to_vec(&value).unwrap_or_else(|e| {
        let fallback = format!(r#"{{"error":"Failed to serialize JSON: {}"}}"#, e);
        fallback.into_bytes()
    });

    Full::new(Bytes::from(json))
        .map_err(|never| match never {})
        .boxed()
}
