// src/lib/routes/health.rs

// dependencies
use crate::state::AppState;
use crate::types::HandlerResult;
use crate::utilities::empty;
use hyper::{Request, Response, body::Incoming};

// health check handler function
pub fn handle_health_check(_req: Request<Incoming>, _state: AppState) -> HandlerResult {
    Box::pin(async move {
        tracing::info!("Health check endpoint reached");
        Ok(Response::new(empty()))
    })
}
