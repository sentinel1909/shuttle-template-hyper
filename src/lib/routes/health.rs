// src/lib/routes/health.rs

// dependencies
use crate::state::AppState;
use crate::types::{HandlerResult, SvcReq, SvcResp};
use crate::utilities::empty;
use hyper::Response;

// health check handler function
pub fn handle_health_check(_request: SvcReq, _state: AppState) -> HandlerResult {
    Box::pin(async move {
        tracing::info!("Health check endpoint reached");
        let response: SvcResp = Response::new(empty());
        Ok(response)
    })
}
