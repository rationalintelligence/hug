use ratatui::Frame;
use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::collections::BTreeMap;

pub struct AppState {
    map: BTreeMap<String, String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn render(&self, frame: &mut Frame<'_>) {
        let data = &self.map;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(frame.area());

        let list = Block::default()
            .borders(Borders::ALL)
            .title("Key-Value Data");
        let inner_area = list.inner(chunks[0]);
        frame.render_widget(list, chunks[0]);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(inner_area);

        let left_column: Vec<Line> = data
            .iter()
            .map(|(key, _)| Line::from(Span::styled(key, Style::default().fg(Color::Yellow))))
            .collect();
        let left_paragraph = Paragraph::new(left_column);

        let right_column: Vec<Line> = data
            .iter()
            .map(|(_, value)| Line::from(Span::styled(value, Style::default().fg(Color::Green))))
            .collect();
        let right_paragraph = Paragraph::new(right_column);

        frame.render_widget(left_paragraph, chunks[0]);
        frame.render_widget(right_paragraph, chunks[1]);
    }
}
