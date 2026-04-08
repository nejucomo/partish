//! Extension traits and other code to make [ratatui::Widget]s more composable
#![deny(unsafe_code, missing_docs)]

pub mod block;
pub mod shelf;
pub mod term;
pub mod widgetext;

pub use self::block::BlockExt;
pub use self::term::TerminalSession;
pub use self::widgetext::WidgetExt;
