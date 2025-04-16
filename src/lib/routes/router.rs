// src/lib/routes/router.rs

// dependencies
use crate::utilities::{empty, response_msg};
use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::{Error, Method, Request, Response, StatusCode};
use tokio::sync::mpsc::Sender;

// type alias for the response type
type RouterResponse = Response<BoxBody<Bytes, Error>>;

// router function; routes incoming requests to send back the appropriate response
pub async fn router(req: Request<Incoming>, ping_tx: Sender<()>) -> Result<RouterResponse, Error> {
    match (req.method(), req.uri().path()) {
        // health_check endpoint
        (&Method::GET, "/_health") => {
            tracing::info!("Health check endpoint reached");
            Ok(Response::new(empty()))
        }

        // ping endpoint
        (&Method::GET, "/ping") => {
            tracing::info!("Ping endpoint reached");

            if let Err(e) = ping_tx.send(()).await {
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
