// src/lib/init.rs

// dependencies
use crate::routes::router_table::RouteTable;
use crate::routes::{handle_count, handle_echo, handle_health_check, handle_metrics, handle_ping};
use crate::types::HandlerFn;
use hyper::Method;

pub fn build_route_table() -> RouteTable {
    let mut table = RouteTable::new();

    table.insert(Method::GET, "/_health", handle_health_check as HandlerFn);
    table.insert(Method::GET, "/ping", handle_ping as HandlerFn);
    table.insert(Method::GET, "/count", handle_count as HandlerFn);
    table.insert(Method::GET, "/metrics", handle_metrics);
    table.insert(Method::GET, "/echo", handle_echo);

    table
}
