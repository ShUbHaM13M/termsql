use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{block::Title, Block, BorderType, Borders, Padding},
};

use crate::{
    app::{App, Screens},
    event::EventHandler,
    utils::centered,
    widgets::dropdown::{Dropdown, DropdownState},
};

use super::Screen;

pub struct ConnectionScreen {
    database_dropdown: Dropdown,
}

impl ConnectionScreen {
    pub fn new() -> Self {
        ConnectionScreen {
            database_dropdown: Dropdown::new(String::from("Database"), vec![]),
        }
    }
}

impl Screen for ConnectionScreen {
    fn render(&self, frame: &mut ratatui::Frame<'_>, area: ratatui::prelude::Rect, app: &App) {
        let container = centered(60, 40, area);
        let block = Block::default()
            .padding(Padding::new(2, 2, 1, 1))
            .title_top(" Database Connection ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner = block.inner(container);
        frame.render_widget(block, container);

        let container = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1)])
            .split(inner);

        let mut dropdown_state = DropdownState {
            selected: app.database,
        };
        frame.render_stateful_widget(
            self.database_dropdown.clone(),
            container[0],
            &mut dropdown_state,
        );
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
