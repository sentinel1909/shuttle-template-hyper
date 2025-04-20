// src/lib/state.rs

// dependencies
use crate::actors::analytics::AnalyticsMessage;
use crate::actors::ping::PingMessage;
use crate::routes::router_table::RouteTable;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

// struct type to represent the application state
#[derive(Debug, Clone)]
pub struct AppState {
    pub analytics_tx: Sender<AnalyticsMessage>,
    pub ping_tx: Sender<PingMessage>,
    pub routes: Arc<RouteTable>,
}
