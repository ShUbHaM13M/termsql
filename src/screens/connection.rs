use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::widgets::{Block, Borders};

use crate::{
    app::{App, Screens},
    event::EventHandler,
};

use super::Screen;

pub struct ConnectionScreen;

impl ConnectionScreen {
    pub fn new() -> Self {
        ConnectionScreen
    }
}

impl Screen for ConnectionScreen {
    fn render(&self, frame: &mut ratatui::Frame<'_>, area: ratatui::prelude::Rect) {
        let block = Block::default().borders(Borders::ALL).title("Home Screen");
        frame.render_widget(block, area);
    }

    fn handle_input(&mut self, app: &mut App, events: &EventHandler) {
        match events.next().unwrap() {
            crate::event::Event::Tick => {}
            crate::event::Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        KeyCode::Char('d') => app.set_screen(Screens::Dashboard),
                        _ => {}
                    }
                }
            }
            crate::event::Event::Mouse(_) => {}
            crate::event::Event::Resize(_, _) => {}
        }
    }
}
