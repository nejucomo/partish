//! [BlockExt] and supporting types for [Block](ratatui::widgets::Block)-centric widget composition
mod ext;

use derive_new::new;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Widget};

pub use self::ext::BlockExt;

/// A [Widget] contained within a [Block]
#[derive(Debug, new)]
pub struct BlockContaining<'b, W>
where
    W: Widget,
{
    block: Block<'b>,
    widget: W,
}

impl<'b, W> Widget for BlockContaining<'b, W>
where
    W: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = self.block.inner(area);
        self.block.render(area, buf);
        self.widget.render(inner, buf);
    }
}
