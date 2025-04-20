// src/lib/routes/router.rs

// dependencies
use crate::actors::analytics::AnalyticsMessage;
use crate::actors::ping::PingMessage;
use crate::errors::ApiError;
use crate::state::AppState;
use crate::utilities::{empty, json_response_msg, set_content_type_json};
use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::{Error, Request, Response, StatusCode};
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::oneshot;

// type aliases
pub type HandlerFn = fn(Request<Incoming>, AppState) -> HandlerResult;
type HandlerResult = Pin<Box<dyn Future<Output = Result<RouterResponse, ApiError>> + Send>>;
type RouterResponse = Response<BoxBody<Bytes, Error>>;

// struct type to represent a response body from the /count endpoint
#[derive(Serialize)]
struct CountResponse {
    count: usize,
}

// struct type to represent a response body from the /ping endpoint
#[derive(Serialize)]
struct PingResponse {
    msg: String,
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

// health check handler function
pub fn handle_health_check(_req: Request<Incoming>, _state: AppState) -> HandlerResult {
    Box::pin(async move {
        tracing::info!("Health check endpoint reached");
        Ok(Response::new(empty()))
    })
}

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

// ping handler function
pub fn handle_ping(_req: Request<Incoming>, state: AppState) -> HandlerResult {
    Box::pin(async move {
        tracing::info!("Ping endpoint reached");

        state
            .ping_tx
            .send(PingMessage::Ping)
            .await
            .map_err(|_| ApiError::ActorUnavailable)?;

        let (tx, rx) = oneshot::channel();

        state
            .analytics_tx
            .send(AnalyticsMessage::Increment {
                key: "ping".to_string(),
                reply: tx,
            })
            .await
            .map_err(|_| ApiError::ActorUnavailable)?;

        rx.await.map_err(|_| ApiError::ActorFailed)?;

        let mut response = Response::new(json_response_msg(PingResponse {
            msg: "Pong".to_string(),
        }));
        set_content_type_json(&mut response);

        Ok(response)
    })
}

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
