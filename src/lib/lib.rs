// src/lib/lib.rs

// dependencies
use http_body_util::{{combinators::BoxBody, BodyExt}, Empty};
use hyper::{Error, Method, StatusCode, Request, Response};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use hyper_util::server::graceful::GracefulShutdown;
use std::net::SocketAddr;
use std::pin::pin;
use tokio::net::TcpListener;
use tokio::signal;

// type alias for the response type
type RouterResponse = Response<BoxBody<Bytes, Error>>;

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
pub struct HyperService {}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for HyperService {
    async fn bind(self, addr: SocketAddr) -> Result<(), shuttle_runtime::Error> {
        // set up a listener, using the Shuttle provided address
        let listener = TcpListener::bind(addr).await?;

        // create a new http instance
        let http = http1::Builder::new();

        // create a new instance of GracefulShutdown
        let graceful = GracefulShutdown::new();

        // pin the shutdown_signal function in memory
        let mut signal = pin!(shutdown_signal());

        // the main loop, listen for incoming connections and serve the router to respond to
        // incoming requests
        loop {
            tokio::select! {
                Ok((stream, _)) = listener.accept() => {
                    let io = TokioIo::new(stream);
                    let conn = http.serve_connection(io, service_fn(router));
                    let fut = graceful.watch(conn);
                    tokio::spawn(async move {
                        if let Err(e) = fut.await {
                            eprintln!("Error serving conection: {:?}", e);
                        }
                    });
                },

                _ = &mut signal => {
                    eprintln!("Graceful shtudown signal received...");
                    break Ok(());
                }
            }
        }
            
    }
}

// utility function which provides a shutdown signal; leverage Tokio::signal
async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install CTRL-C signal handler");
}

// router function; routes incomign requests to send back the appropriate response
async fn router(req: Request<Incoming>,) -> Result<RouterResponse, Error> {
    match (req.method(), req.uri().path()) {
        
        // health_check endpoint
        (&Method::GET, "/health_check") => Ok(Response::new(empty())),

        // 404 Not Found; for any non-matching routes
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

// utility function to create an empty body for a Response
fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}