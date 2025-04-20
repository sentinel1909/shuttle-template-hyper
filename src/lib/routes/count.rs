// src/lib/routes/count.rs

// dependencies
use crate::actors::ping::PingMessage;
use crate::errors::ApiError;
use crate::state::AppState;
use crate::types::{HandlerResult, SvcReq, SvcResp};
use crate::utilities::{json_response_msg, set_content_type_json};
use hyper::Response;
use tokio::sync::oneshot;

// count handler function
pub fn handle_count(_request: SvcReq, state: AppState) -> HandlerResult {
    Box::pin(async move {
        tracing::info!("Count endpoint reached");

        let (tx, rx) = oneshot::channel();
        state
            .ping_tx
            .send(PingMessage::GetCount(tx))
            .await
            .map_err(|_| ApiError::ActorUnavailable)?;

        let count = rx.await?;

        let mut response: SvcResp = Response::new(json_response_msg(count));
        set_content_type_json(&mut response);

        Ok(response)
    })
}
