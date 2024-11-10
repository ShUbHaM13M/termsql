use ratatui::{style::Stylize, widgets::Paragraph, DefaultTerminal};

use crate::{
    event::EventHandler,
    screens::{connection::ConnectionScreen, dashboard::DashboardScreen, Screen},
};

pub enum Screens {
    Connection,
    Dashboard,
}

pub struct App {
    pub current_screen: Screens,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: Screens::Connection,
            should_quit: false,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        let events = EventHandler::new(250);
        while !self.should_quit {
            let mut screen: Option<Box<dyn Screen>> = match self.current_screen {
                Screens::Connection => Some(Box::new(ConnectionScreen)),
                Screens::Dashboard => Some(Box::new(DashboardScreen)),
            };
            terminal.draw(|frame| {
                if screen.is_some() {
                    screen.as_ref().unwrap().render(frame, frame.area());
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
