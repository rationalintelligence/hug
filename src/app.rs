use crate::events::EventsDrainer;
use crate::state::AppState;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, DoSync, Duty, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::{Supervisor, SupervisorSession};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    DefaultTerminal,
};

pub struct HubApp {
    terminal: Slot<DefaultTerminal>,
    state: AppState,
}

impl HubApp {
    pub fn new() -> Self {
        Self {
            terminal: Slot::empty(),
            state: AppState::new(),
        }
    }
}

impl Supervisor for HubApp {
    type GroupBy = ();
}

impl Agent for HubApp {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Configure)
    }
}

struct Configure;

#[async_trait]
impl Duty<Configure> for HubApp {
    async fn handle(&mut self, _: Configure, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let terminal = ratatui::try_init()?;
        self.terminal.fill(terminal)?;
        let drainer = EventsDrainer::new(&ctx);
        ctx.spawn_agent(drainer, ());
        Ok(Next::do_sync(Render))
    }
}

#[async_trait]
impl OnEvent<Event> for HubApp {
    async fn handle(&mut self, event: Event, ctx: &mut Context<Self>) -> Result<()> {
        let next_state = match event {
            Event::Key(event) => match event.code {
                KeyCode::Char('q') => Next::do_async(Terminate),
                _ => Next::do_sync(Render),
            },
            _ => Next::do_sync(Render),
        };
        ctx.do_next(next_state);
        Ok(())
    }
}

struct Render;

impl DoSync<Render> for HubApp {
    fn once(&mut self, _: &mut Render) -> Result<Next<Self>> {
        let terminal = self.terminal.get_mut()?;

        let data = vec![("Key1", "Value1"), ("Key2", "Value2"), ("Key3", "Value3")];

        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(frame.size());

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
                .map(|(key, _)| Line::from(Span::styled(*key, Style::default().fg(Color::Yellow))))
                .collect();
            let left_paragraph = Paragraph::new(left_column);

            let right_column: Vec<Line> = data
                .iter()
                .map(|(_, value)| {
                    Line::from(Span::styled(*value, Style::default().fg(Color::Green)))
                })
                .collect();
            let right_paragraph = Paragraph::new(right_column);

            frame.render_widget(left_paragraph, chunks[0]);
            frame.render_widget(right_paragraph, chunks[1]);
        })?;

        Ok(Next::events())
    }
}

struct Terminate;

#[async_trait]
impl DoAsync<Terminate> for HubApp {
    async fn once(&mut self, _: &mut Terminate) -> Result<Next<Self>> {
        ratatui::try_restore()?;
        Ok(Next::done())
    }
}
