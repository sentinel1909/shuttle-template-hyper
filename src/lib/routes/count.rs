// src/lib/routes/count.rs

// dependencies
use crate::actors::ping::PingMessage;
use crate::errors::ApiError;
use crate::state::AppState;
use crate::types::HandlerResult;
use crate::utilities::{json_response_msg, set_content_type_json};
use hyper::{Request, Response, body::Incoming};
use serde::Serialize;
use tokio::sync::oneshot;

// struct type to represent a response body from the /count endpoint
#[derive(Serialize)]
struct CountResponse {
    count: usize,
}

// count handler function
pub fn handle_count(_request: Request<Incoming>, state: AppState) -> HandlerResult {
    Box::pin(async move {
        tracing::info!("Count endpoint reached");

        let (tx, rx) = oneshot::channel();
        state
            .ping_tx
            .send(PingMessage::GetCount(tx))
            .await
            .map_err(|_| ApiError::ActorUnavailable)?;

        let count = rx.await?;

        let mut response = Response::new(json_response_msg(CountResponse { count }));
        set_content_type_json(&mut response);

        Ok(response)
    })
}
