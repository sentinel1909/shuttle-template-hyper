// src/lib/routes/router_table.rs

// dependencies
use crate::types::HandlerFn;
use hyper::Method;
use matchit::{Params, Router as MatchitRouter};
use std::collections::HashMap;

// struct type to store route definitions by method and path
#[derive(Debug)]
pub struct RouteTable {
    routes: HashMap<Method, MatchitRouter<HandlerFn>>,
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
        handler: HandlerFn, // we'll link this to real handler funcs later
    ) {
        self.routes
            .entry(method)
            .or_default()
            .insert(path, handler)
            .expect("failed to insert route");
    }

    pub fn at<'a>(&'a self, method: &Method, path: &'a str) -> Option<(HandlerFn, Params<'a, 'a>)> {
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
