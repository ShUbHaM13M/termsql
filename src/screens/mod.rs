use ratatui::{prelude::Rect, Frame};

use crate::{app::App, event::EventHandler};

pub mod connection;
pub mod dashboard;

pub trait Screen {
    fn render(&self, frame: &mut Frame<'_>, area: Rect, app: &App);
    fn handle_input(&mut self, app: &mut App, event: &EventHandler);
}
