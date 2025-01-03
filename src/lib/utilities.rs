// src/lib/utilities.rs

// dependencies
use http_body_util::{
    Empty,
    {combinators::BoxBody, BodyExt},
};
use hyper::body::Bytes;
use tokio::signal;

// utility function which provides a shutdown signal; leverage Tokio::signal
pub async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install CTRL-C signal handler");
}

// utility function to create an empty body for a Response
pub fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
