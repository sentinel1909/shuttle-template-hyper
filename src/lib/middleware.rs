// src/lib/middleware.rs

// dependencies
use hyper::{Request, body::Incoming, service::Service};

// struct type to represent a Logger service
#[derive(Debug, Clone)]
pub struct Logger<S> {
    inner: S,
}

// methods for the Logger service
impl<S> Logger<S> {
    pub fn new(inner: S) -> Self {
        Logger { inner }
    }
}

// convenience type alias
type Req = Request<Incoming>;

// implement the Service trait for the Logger type
impl<S> Service<Req> for Logger<S>
where
    S: Service<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;
    fn call(&self, req: Req) -> Self::Future {
        println!("processing request: {} {}", req.method(), req.uri().path());
        self.inner.call(req)
    }
}
