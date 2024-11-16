use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

use crate::app::DatabaseType;

#[derive(Clone)]
pub struct Dropdown {
    label: String,
    options: Vec<String>,
    expanded: bool,
}

pub struct DropdownState {
    pub selected: Option<DatabaseType>,
}

impl Dropdown {
    pub fn new(label: String, options: Vec<String>) -> Self {
        Self {
            label,
            options,
            expanded: false,
        }
    }
}

impl StatefulWidget for Dropdown {
    type State = DropdownState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(self.label);
        block.render(area, buf);
        let mut selected = "Select database type";

        if state.selected.is_some() {
            selected = match state.selected.unwrap() {
                DatabaseType::MySql => "MySql",
                DatabaseType::PostgreSql => "PostgreSql",
                DatabaseType::Sqlite => "Sqlite",
            };
        }

        buf.set_string(area.x + 2, area.y + 1, selected, Style::default());
    }
}
