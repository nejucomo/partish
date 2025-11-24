//! [Shelf] and supporting types for single-axis layout

mod shelved;
mod shelving;
use std::rc::Rc;

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub use self::shelved::Shelved;
pub use self::shelving::Shelving;

/// A [Shelf] is used to contain widgets to-be layed out on one [Direction]al axis
#[derive(Debug)]
pub struct Shelf {
    layout: Layout,
    constraints: Vec<Constraint>,
}

impl Shelf {
    /// Construct a new shelf along the given axis
    pub fn new(dir: Direction) -> Self {
        let no_constraints: [Constraint; 0] = [];
        Shelf {
            layout: Layout::new(dir, no_constraints),
            constraints: vec![],
        }
    }
}

impl shelving::Sealed for Shelf {
    fn map_layout<F>(self, f: F) -> Self
    where
        F: FnOnce(Layout) -> Layout,
    {
        Shelf {
            layout: f(self.layout),
            constraints: self.constraints,
        }
    }

    fn append_constraint(&mut self, constraint: Constraint) -> usize {
        let ix = self.constraints.len();
        self.constraints.push(constraint);
        ix
    }

    fn render_shelving(self, area: Rect, _: &mut Buffer) -> Rc<[Rect]> {
        self.layout.constraints(self.constraints).split(area)
    }
}
