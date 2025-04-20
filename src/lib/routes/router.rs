// src/lib/routes/router.rs

// dependencies

use crate::errors::ApiError;
use crate::state::AppState;
use crate::types::RouterResponse;
use crate::utilities::empty;
use hyper::body::Incoming;
use hyper::{Request, Response, StatusCode};

// router function; routes incoming requests to send back the appropriate response
pub async fn router(
    request: Request<Incoming>,
    state: AppState,
) -> Result<RouterResponse, ApiError> {
    let method = request.method();
    let path = request.uri().path();
    match state.routes.at(method, path) {
        Some((handler_fn, _params)) => handler_fn(request, state).await,
        None => {
            tracing::info!("Not found handler reached");
            let mut not_found_response = Response::new(empty());
            *not_found_response.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found_response)
        }
    }
}
