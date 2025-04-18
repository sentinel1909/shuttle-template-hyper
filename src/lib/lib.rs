// src/lib/lib.rs

// module declarations
pub mod actors;
pub mod errors;
pub mod routes;
pub mod service;
pub mod utilities;

// re-exports
pub use actors::*;
pub use errors::*;
pub use service::*;
pub use utilities::*;
