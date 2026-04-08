//! [StreamDispatcher] and associated helper code
#![deny(unsafe_code, missing_docs)]

mod handler;
mod sdisp;

pub use self::handler::Handler;
pub use self::sdisp::StreamHandler;
