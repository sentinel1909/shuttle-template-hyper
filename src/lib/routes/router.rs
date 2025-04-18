// src/lib/routes/router.rs

// dependencies
use crate::actors::PingMessage;
use crate::errors::ApiError;
use crate::utilities::{empty, json_response_msg, set_content_type_json};
use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::{Error, Method, Request, Response, StatusCode};
use serde::Serialize;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

// type alias for the response type
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

// router function; routes incoming requests to send back the appropriate response
pub async fn router(
    req: Request<Incoming>,
    ping_tx: Sender<PingMessage>,
) -> Result<RouterResponse, ApiError> {
    match (req.method(), req.uri().path()) {
        // health_check endpoint
        (&Method::GET, "/_health") => {
            tracing::info!("Health check endpoint reached");
            Ok(Response::new(empty()))
        }

        // count endpoint
        (&Method::GET, "/count") => {
            tracing::info!("Count endpoint reached");

            let (tx, rx) = oneshot::channel();

            ping_tx
                .send(PingMessage::GetCount(tx))
                .await
                .map_err(|_| ApiError::ActorUnavailable)?;

            let count = rx.await?;

            let mut response = Response::new(json_response_msg(CountResponse { count }));

            set_content_type_json(&mut response);

            Ok(response)
        }

        // ping endpoint
        (&Method::GET, "/ping") => {
            tracing::info!("Ping endpoint reached");

            ping_tx
                .send(PingMessage::Ping)
                .await
                .map_err(|_| ApiError::ActorUnavailable)?;

            let mut response = Response::new(json_response_msg(PingResponse {
                msg: "Pong".to_string(),
            }));
            set_content_type_json(&mut response);

            Ok(response)
        }

        // 404 Not Found; for any non-matching routes
        _ => {
            tracing::info!("Not found handler reached");
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
