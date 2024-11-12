// src/lib/routes/router.rs

// dependencies
use crate::utilities::empty;
use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::{Error, Method, Request, Response, StatusCode};

// type alias for the response type
type RouterResponse = Response<BoxBody<Bytes, Error>>;

// router function; routes incomign requests to send back the appropriate response
pub async fn router(req: Request<Incoming>) -> Result<RouterResponse, Error> {
    match (req.method(), req.uri().path()) {
        // health_check endpoint
        (&Method::GET, "/health_check") => Ok(Response::new(empty())),

        // 404 Not Found; for any non-matching routes
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
