// This macro includes the scaffolding for the Iroh FFI bindings.
uniffi::setup_scaffolding!();

mod error;
mod gossip;

pub use self::error::*;
pub use self::gossip::*;
