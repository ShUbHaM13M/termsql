use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::widgets::{Block, Borders};

use crate::{app::App, event::EventHandler};

use super::Screen;

pub struct DashboardScreen;

impl DashboardScreen {
    pub fn new() -> Self {
        DashboardScreen
    }
}

impl Screen for DashboardScreen {
    fn render(&self, frame: &mut ratatui::Frame<'_>, area: ratatui::prelude::Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Dashboard Screen");
        frame.render_widget(block, area);
    }

    fn handle_input(&mut self, app: &mut App, events: &EventHandler) {
        match events.next().unwrap() {
            crate::event::Event::Tick => {}
            crate::event::Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        _ => {}
                    }
                }
            }
            crate::event::Event::Mouse(_) => {}
            crate::event::Event::Resize(_, _) => {}
        }
    }
}
