//! [TerminalSession] for RAII terminal lifecycle management
use ratatui::crossterm::ExecutableCommand as _;
use ratatui::crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use ratatui::widgets::Widget;
use ratatui::{CompletedFrame, DefaultTerminal};

/// Provides [RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization) access to the terminal alternate screen; dropping causes a reset#[derive(Debug)]
pub struct TerminalSession(DefaultTerminal);

impl TerminalSession {
    /// Initialize the session
    pub fn start() -> std::io::Result<Self> {
        let me = TerminalSession(ratatui::init());
        std::io::stdout().execute(PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::REPORT_EVENT_TYPES,
        ))?;
        Ok(me)
    }

    /// Draw a widget onto the terminal
    pub fn draw<W>(&mut self, widget: W) -> std::io::Result<CompletedFrame<'_>>
    where
        W: Widget,
    {
        self.0
            .draw(|frame| frame.render_widget(widget, frame.area()))
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        std::io::stdout()
            .execute(PopKeyboardEnhancementFlags)
            .unwrap();
        ratatui::restore();
    }
}
