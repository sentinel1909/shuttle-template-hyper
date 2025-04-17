// src/lib/routes/router.rs

// dependencies
use crate::actors::PingMessage;
use crate::utilities::{empty, response_msg};
use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::{Error, Method, Request, Response, StatusCode};
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

// type alias for the response type
type RouterResponse = Response<BoxBody<Bytes, Error>>;

// router function; routes incoming requests to send back the appropriate response
pub async fn router(
    req: Request<Incoming>,
    ping_tx: Sender<PingMessage>,
) -> Result<RouterResponse, Error> {
    match (req.method(), req.uri().path()) {
        // health_check endpoint
        (&Method::GET, "/_health") => {
            tracing::info!("Health check endpoint reached");
            Ok(Response::new(empty()))
        }

        (&Method::GET, "/count") => {
            tracing::info!("Count endpoint reached");

            let (tx, rx) = oneshot::channel();

            if let Err(e) = ping_tx.send(PingMessage::GetCount(tx)).await {
                tracing::error!("Failed to send GetCount to actor: {e}");
                let mut resp = Response::new(response_msg("Actor unavailable"));
                *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                return Ok(resp);
            }

            let count = match rx.await {
                Ok(count) => count,
                Err(e) => {
                    tracing::error!("Actor didn't reply: {e}");
                    let mut resp = Response::new(response_msg("Actor did not respond"));
                    *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    return Ok(resp);
                }
            };

            Ok(Response::new(response_msg(count.to_string())))
        }

        // ping endpoint
        (&Method::GET, "/ping") => {
            tracing::info!("Ping endpoint reached");

            if let Err(e) = ping_tx.send(PingMessage::Ping).await {
                tracing::error!("Failed to send ping to actor: {}", e);
            }

            Ok(Response::new(response_msg("Pong")))
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
