//! [WidgetExt] and supporting types
mod within_block;

use ratatui::widgets::{Block, Widget};

pub use crate::widgetext::within_block::WithinBlock;

/// An extension to [Widget] enabling composable methods
pub trait WidgetExt: Sized + Widget {
    /// Contain `self` within the given [Block]
    fn within_block<'b>(self, b: Block<'b>) -> WithinBlock<'b, Self>;
}

impl<W> WidgetExt for W
where
    W: Widget,
{
    fn within_block<'b>(self, b: Block<'b>) -> WithinBlock<'b, Self> {
        WithinBlock::new(self, b)
    }
}
