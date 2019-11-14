// This crate serves to house stuff common between client and server, including external dependencies
mod ipc;

pub mod prelude {
    pub use crate::ipc::*;

    pub use ws;
    pub use ron;
    pub use chrono;
    pub use chrono::prelude::*;
    pub use serde;
}