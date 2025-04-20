// src/lib/routes/mod.rs

// module declarations
pub mod count;
pub mod echo;
pub mod health;
pub mod metrics;
pub mod ping;
pub mod router;
pub mod router_table;

// re-exports
pub use count::*;
pub use echo::*;
pub use health::*;
pub use metrics::*;
pub use ping::*;
pub use router::*;
pub use router_table::*;
