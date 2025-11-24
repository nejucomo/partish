use ratatui::widgets::{Block, Widget};

use crate::block::BlockContaining;

/// Extension trait for [Block]
pub trait BlockExt<'b> {
    /// Return `self` containing `widget`
    fn containing<W>(self, widget: W) -> BlockContaining<'b, W>
    where
        W: Widget;
}

impl<'b> BlockExt<'b> for Block<'b> {
    fn containing<W>(self, widget: W) -> BlockContaining<'b, W>
    where
        W: Widget,
    {
        BlockContaining::new(self, widget)
    }
}
