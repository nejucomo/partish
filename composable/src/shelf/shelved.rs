use std::rc::Rc;

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::Widget;

use crate::shelf::Shelving;
use crate::shelf::shelving::Sealed;

/// A [Widget] stacked on a [Shelf](super::Shelf) (via [Shelving])
#[derive(Debug)]
pub struct Shelved<S, W>
where
    S: Shelving,
    W: Widget,
{
    ix: usize,
    sh: S,
    w: W,
}

impl<S, W> Shelved<S, W>
where
    S: Shelving,
    W: Widget,
{
    pub(super) fn new(mut sh: S, cons: Constraint, w: W) -> Self {
        let ix = sh.append_constraint(cons);
        Shelved { ix, sh, w }
    }
}

impl<S, W> Sealed for Shelved<S, W>
where
    S: Shelving,
    W: Widget,
{
    fn append_constraint(&mut self, constraint: Constraint) -> usize {
        self.sh.append_constraint(constraint)
    }

    fn render_shelving(self, area: Rect, buf: &mut Buffer) -> Rc<[Rect]> {
        let Shelved { ix, sh, w } = self;
        let areas = sh.render_shelving(area, buf);
        w.render(areas[ix], buf);
        areas
    }
}

impl<S, W> Widget for Shelved<S, W>
where
    S: Shelving,
    W: Widget,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_shelving(area, buf);
    }
}
