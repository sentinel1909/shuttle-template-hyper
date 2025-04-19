// tests/api/helpers.rs

// dependencies
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use reqwest::Client;
use shuttle_hyper_template_lib::init::build_route_table;
use shuttle_hyper_template_lib::routes::router;
use shuttle_hyper_template_lib::{AppState, PingCounterActor};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

// start a test server, with a sender, and return the address it is listening on
pub async fn start_test_server_with_state(state: AppState) -> SocketAddr {
    let address = "127.0.0.1";
    let port: u16 = 0;
    let socket = format!("{}:{}", address, port);
    let listener = TcpListener::bind(socket)
        .await
        .expect("Unable to create a listener.");
    let addr = listener
        .local_addr()
        .expect("Unable to obtain the address the test server is using");

    tokio::spawn(async move {
        loop {
            let (stream, _) = listener
                .accept()
                .await
                .expect("Unable to listen for an incoming stream.");
            let io = TokioIo::new(stream);
            let state = state.clone();
            tokio::spawn(async move {
                let svc = service_fn(move |req| {
                    let state = state.clone();
                    async move {
                        match router(req, state).await {
                            Ok(resp) => Ok::<_, hyper::Error>(resp),
                            Err(api_err) => Ok(api_err.to_response()),
                        }
                    }
                });

                http1::Builder::new()
                    .serve_connection(io, svc)
                    .await
                    .expect("Unable to start the server to listen for requests.")
            });
        }
    });

    addr
}

// start a server
pub async fn start_test_server() -> SocketAddr {
    let (tx, _handle) = PingCounterActor::start();

    let state = AppState {
        routes: Arc::new(build_route_table()),
        ping_tx: tx,
    };

    start_test_server_with_state(state).await
}

// helper function to build a test client
pub fn get_test_client() -> Client {
    Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Unable to build test client.")
}
