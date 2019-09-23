#![allow(missing_docs)]

//! TODO: write transport docs.

pub mod channel;
pub mod server;

mod endpoint;
mod error;
mod service;
mod tls;

pub use self::channel::Channel;
pub use self::endpoint::Endpoint;
pub use self::error::Error;
pub use self::server::Server;
pub use hyper::Body;

pub(crate) use self::error::ErrorKind;
