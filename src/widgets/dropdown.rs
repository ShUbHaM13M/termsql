use ratatui::widgets::{StatefulWidget, Widget};

pub struct Dropdown {
    label: String,
    options: Vec<String>,
    expanded: bool,
}

pub struct DropdownState {
    selected: Option<u32>,
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
        todo!()
    }
}
