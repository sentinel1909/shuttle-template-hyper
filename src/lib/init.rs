// src/lib/init.rs

// dependencies
use crate::routes::router::HandlerFn;
use crate::routes::router_table::RouteTable;
use crate::routes::{handle_count, handle_health_check, handle_ping};
use hyper::Method;

pub fn build_route_table() -> RouteTable {
    let mut table = RouteTable::new();

    table.insert(Method::GET, "/_health", handle_health_check as HandlerFn);
    table.insert(Method::GET, "/ping", handle_ping as HandlerFn);
    table.insert(Method::GET, "/count", handle_count as HandlerFn);

    table
}
