use ratatui::{style::Stylize, widgets::Paragraph, DefaultTerminal};

use crate::{
    event::EventHandler,
    screens::{connection::ConnectionScreen, dashboard::DashboardScreen, Screen},
};

pub enum Screens {
    Connection,
    Dashboard,
}

// TODO: Move to another file
#[derive(Clone, Copy)]
pub enum DatabaseType {
    MySql,
    PostgreSql,
    Sqlite,
}

pub struct App {
    pub current_screen: Screens,
    pub database: Option<DatabaseType>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: Screens::Connection,
            should_quit: false,
            database: None,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        let events = EventHandler::new(250);
        while !self.should_quit {
            let mut screen: Option<Box<dyn Screen>> = match self.current_screen {
                Screens::Connection => Some(Box::new(ConnectionScreen::new())),
                Screens::Dashboard => Some(Box::new(DashboardScreen)),
            };
            terminal.draw(|frame| {
                if screen.is_some() {
                    screen.as_ref().unwrap().render(frame, frame.area(), &self);
                } else {
                    let greeting = Paragraph::new("Hello Ratatui! {} (press 'q' to quit)")
                        .white()
                        .on_blue();
                    frame.render_widget(greeting, frame.area());
                }
            })?;
            screen.as_mut().unwrap().handle_input(self, &events);
        }
        Ok(())
    }

    pub fn set_screen(&mut self, screen: Screens) {
        self.current_screen = screen;
    }
}
