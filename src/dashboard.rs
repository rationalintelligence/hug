use super::State;
use anyhow::Error;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Constraint;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Row, Table};
use ratatui::Terminal;
use std::io;
use std::thread;
use std::time::Duration;

pub struct Dashboard {
    state: State,
}

impl Dashboard {
    pub fn new(state: State) -> Self {
        Self { state }
    }

    pub fn spawn(mut self) {
        thread::spawn(move || {
            self.init().ok();
            self.render().ok();
            self.uninit().ok();
        });
    }

    fn init(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        Ok(())
    }

    fn uninit(&mut self) -> Result<(), Error> {
        let mut stdout = io::stdout();
        disable_raw_mode()?;
        stdout.execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn render(&mut self) -> Result<(), Error> {
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;

        // TODO: Add an atomic
        loop {
            {
                let pairs = self.state.blocking_read();
                terminal.draw(|f| {
                    let size = f.area();

                    let rows: Vec<Row> = pairs
                        .iter()
                        .map(|(key, value)| Row::new(vec![key.to_string(), value.to_string()]))
                        .collect();

                    let table = Table::new(
                        rows,
                        [Constraint::Percentage(10), Constraint::Percentage(90)],
                    )
                    .header(
                        Row::new(vec!["Key", "Value"]).style(Style::default().fg(Color::Yellow)),
                    )
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("CLI Dashboard"),
                    )
                    .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

                    f.render_widget(table, size);
                })?;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
}
