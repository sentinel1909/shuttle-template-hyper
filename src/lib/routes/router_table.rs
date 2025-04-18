// src/lib/routes/router_table.rs

// dependencies
use hyper::Method;
use matchit::{Params, Router as MatchitRouter};
use std::collections::HashMap;

// enum type to represent route handlers
#[derive(Debug, Clone, Copy)]
pub enum RouteHandler {
    HealthCheck,
    Ping,
    Count,
}

// struct type to store route definitions by method and path
pub struct RouteTable {
    routes: HashMap<Method, MatchitRouter<RouteHandler>>,
}

// methods for the RouterTable type
impl RouteTable {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        method: Method,
        path: &'static str,
        handler: RouteHandler, // we'll link this to real handler funcs later
    ) {
        self.routes
            .entry(method)
            .or_default()
            .insert(path, handler)
            .expect("failed to insert route");
    }

    pub fn at<'a>(
        &'a self,
        method: &Method,
        path: &'a str,
    ) -> Option<(RouteHandler, Params<'a, 'a>)> {
        self.routes
            .get(method)
            .and_then(|router| router.at(path).ok().map(|m| (*m.value, m.params)))
    }
}

// implement the Default trait for RouteTable
impl Default for RouteTable {
    fn default() -> Self {
        Self::new()
    }
}
