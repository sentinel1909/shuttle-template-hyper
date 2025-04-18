// src/lib/routes/router.rs

// dependencies
use crate::actors::PingMessage;
use crate::errors::ApiError;
use crate::routes::router_table::{RouteHandler, RouteTable};
use crate::utilities::{empty, json_response_msg, set_content_type_json};
use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::{Error, Method, Request, Response, StatusCode};
use serde::Serialize;
use std::sync::OnceLock;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

// type alias for the response type
type RouterResponse = Response<BoxBody<Bytes, Error>>;

// declare a static routes constant
static ROUTES: OnceLock<RouteTable> = OnceLock::new();

// function to create the routes
fn route_table() -> &'static RouteTable {
    ROUTES.get_or_init(|| {
        let mut table = RouteTable::new();

        table.insert(Method::GET, "/_health", RouteHandler::HealthCheck);
        table.insert(Method::GET, "/ping", RouteHandler::Ping);
        table.insert(Method::GET, "/count", RouteHandler::Count);

        table
    })
}

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
    let method = req.method().clone();
    let path = req.uri().path();
    match route_table().at(&method, path) {
        Some((RouteHandler::HealthCheck, _)) => {
            tracing::info!("Health check endpoint reached");
            Ok(Response::new(empty()))
        }

        Some((RouteHandler::Ping, _)) => {
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

        Some((RouteHandler::Count, _)) => {
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

        None => {
            tracing::info!("Not found handler reached");
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
