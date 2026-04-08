use std::rc::Rc;

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Layout, Rect, Spacing};
use ratatui::widgets::Widget;

use crate::shelf::Shelved;

/// A sealed trait to enable stacking widgets upon both [Shelf](super::Shelf) and [Shelved](super::Shelved)
pub trait Shelving: Sized + Sealed {
    /// Stack a `widget` onto the shelf from left-to-right for [Horizontal](Direction::Horizontal) shelves and top-to-bottom for [Vertical](Direction::Vertical) shelves
    fn stack<W>(self, constraint: Constraint, widget: W) -> Shelved<Self, W>
    where
        W: Widget,
    {
        Shelved::new(self, constraint, widget)
    }

    /// Same meaning as [Layout::margin]
    fn margin(self, margin: u16) -> Self {
        self.map_layout(|layout| layout.margin(margin))
    }

    /// Same meaning as [Layout::horizontal_margin]
    fn horizontal_margin(self, horizontal: u16) -> Self {
        self.map_layout(|layout| layout.horizontal_margin(horizontal))
    }

    /// Same meaning as [Layout::vertical_margin]
    fn vertical_margin(self, vertical: u16) -> Self {
        self.map_layout(|layout| layout.vertical_margin(vertical))
    }

    /// Same meaning as [Layout::flex]
    fn flex(self, flex: Flex) -> Self {
        self.map_layout(|layout| layout.flex(flex))
    }

    /// Same meaning as [Layout::spacing]
    fn spacing<T>(self, spacing: T) -> Self
    where
        T: Into<Spacing>,
    {
        self.map_layout(|layout| layout.spacing(spacing))
    }
}

impl<T> Shelving for T where T: Sealed {}

pub trait Sealed {
    fn map_layout<F>(self, f: F) -> Self
    where
        F: FnOnce(Layout) -> Layout;

    fn append_constraint(&mut self, constraint: Constraint) -> usize;

    fn render_shelving(self, area: Rect, buf: &mut Buffer) -> Rc<[Rect]>;
}
