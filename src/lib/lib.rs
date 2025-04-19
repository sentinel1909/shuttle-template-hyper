// src/lib/lib.rs

// module declarations
pub mod actors;
pub mod errors;
pub mod init;
pub mod routes;
pub mod service;
pub mod state;
pub mod utilities;

// re-exports
pub use actors::*;
pub use errors::*;
pub use init::*;
pub use service::*;
pub use state::*;
pub use utilities::*;
