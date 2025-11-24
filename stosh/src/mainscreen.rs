use ratatui::buffer::Buffer;
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::layout::{Direction::Vertical, Rect};
use ratatui::style::{Style, Stylize as _};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Widget};
use ratatui_composable::WidgetExt as _;
use ratatui_composable::shelf::{Shelf, Shelving as _};

use crate::cmd;
use crate::event::{ControlMessage, InputEvent};
use crate::handler::Handler;
use crate::u16util::IntoU16 as _;

#[derive(Debug, Default)]
pub(crate) struct MainScreen {
    input: cmd::Input,
    stack: cmd::Stack,
}

impl Handler<InputEvent> for MainScreen {
    type Response = ControlMessage;

    fn handle(&mut self, ev: InputEvent) -> ControlMessage {
        use ControlMessage::LaunchCommand;
        use InputEvent::*;

        match ev {
            Terminal(termev) => match self.input.handle(termev) {
                LaunchCommand(h, cmdlines) => {
                    self.stack.push(cmd::Portal::new(h, cmdlines.clone()));
                    LaunchCommand(h, cmdlines)
                }
                other => other,
            },
            Child(childev) => {
                self.stack.handle(childev);
                ControlMessage::NoCtrl
            }
        }
    }
}

impl Widget for &MainScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Shelf::new(Vertical)
            .stack(Length(self.input.height().into_u16()), &self.input)
            .stack(Fill(1), &self.stack)
            .within_block(
                Block::new()
                    .title_top(Line::from("stosh").light_green().right_aligned())
                    .borders(Borders::TOP)
                    .border_style(Style::new().green()),
            )
            .render(area, buf)
    }
}
