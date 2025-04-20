// src/lib/routes/metrics.rs

// dependencies
use crate::errors::ApiError;
use crate::state::AppState;
use crate::types::HandlerResult;
use crate::utilities::{json_response_msg, set_content_type_json};
use hyper::{Request, Response, body::Incoming};
use tokio::sync::oneshot;

// metrics handler function
pub fn handle_metrics(_req: Request<Incoming>, state: AppState) -> HandlerResult {
    Box::pin(async move {
        let (tx, rx) = oneshot::channel();

        state
            .analytics_tx
            .send(crate::AnalyticsMessage::GetAll { reply: tx })
            .await
            .map_err(|_| ApiError::ActorUnavailable)?;

        let data = rx.await.map_err(|_| ApiError::ActorFailed)?;

        let mut response = Response::new(json_response_msg(data));
        set_content_type_json(&mut response);

        Ok(response)
    })
}
