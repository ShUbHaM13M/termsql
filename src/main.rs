use crate::app::App;

pub mod app;
pub mod event;
pub mod screens;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let result = app.run(terminal);
    ratatui::restore();
    result
}
