// src/lib/routes/health.rs

// dependencies
use crate::state::AppState;
use crate::types::{HandlerResult, SvcReq, SvcResp};
use crate::utilities::json_response_msg;
use hyper::Response;

// health check handler function
pub fn handle_health_check(_request: SvcReq, _state: AppState) -> HandlerResult {
    Box::pin(async move {
        tracing::info!("Health check endpoint reached");
        let response_msg = "server available";
        let response: SvcResp = Response::new(json_response_msg(response_msg));
        Ok(response)
    })
}
