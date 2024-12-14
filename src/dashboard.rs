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
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct Dashboard {
    active: Arc<AtomicBool>,
    handle: JoinHandle<()>,
}

impl Dashboard {
    pub fn start(state: State) -> Self {
        let active = Arc::new(AtomicBool::new(true));
        let task = DashboardTask {
            active: active.clone(),
            state,
        };
        let handle = task.spawn();
        Self { active, handle }
    }

    pub fn stop(&mut self) {
        self.active.store(false, Ordering::Relaxed);
    }

    pub fn join(self) {
        self.handle.join().ok();
    }
}

struct DashboardTask {
    active: Arc<AtomicBool>,
    state: State,
}

impl DashboardTask {
    pub fn spawn(mut self) -> JoinHandle<()> {
        thread::spawn(move || {
            self.init().ok();
            self.render().ok();
            self.uninit().ok();
        })
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
        while self.active.load(Ordering::Relaxed) {
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
                    .block(Block::default().borders(Borders::ALL).title("Dashboard"))
                    .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

                    f.render_widget(table, size);
                })?;
            }
            thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }
}
