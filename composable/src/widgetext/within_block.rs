use derive_new::new;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Widget};

/// A [Block] containing a [Widget]
#[derive(Debug, new)]
pub struct WithinBlock<'b, W>
where
    W: Widget,
{
    widget: W,
    block: Block<'b>,
}

impl<'b, W> Widget for WithinBlock<'b, W>
where
    W: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = self.block.inner(area);
        self.block.render(area, buf);
        self.widget.render(inner, buf);
    }
}
