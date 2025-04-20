// src/lib/types.rs

// dependencies
use crate::errors::ApiError;
use crate::state::AppState;
use http_body_util::combinators::BoxBody;
use hyper::{
    Error, Request, Response,
    body::{Bytes, Incoming},
};
use std::{future::Future, pin::Pin};

// type aliases
pub(crate) type HandlerFn = fn(Request<Incoming>, AppState) -> HandlerResult;
pub(crate) type HandlerResult =
    Pin<Box<dyn Future<Output = Result<RouterResponse, ApiError>> + Send>>;
pub(crate) type RouterResponse = Response<BoxBody<Bytes, Error>>;
