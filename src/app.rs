use crate::args::RunArgs;
use crate::events::EventsDrainer;
use crate::launcher::{CommandEvent, CommandWatcher};
use crate::state::AppState;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, DoSync, Duty, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::{Supervisor, SupervisorSession};
use crossterm::event::{Event, KeyCode};
use ratatui::DefaultTerminal;

pub struct HubApp {
    args: RunArgs,
    terminal: Slot<DefaultTerminal>,
    state: AppState,
}

impl HubApp {
    pub fn new(args: RunArgs) -> Self {
        Self {
            args,
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

        let (watcher, _tx) = CommandWatcher::new(self.args.clone(), &ctx);
        ctx.spawn_agent(watcher, ());

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

#[async_trait]
impl OnEvent<CommandEvent> for HubApp {
    async fn handle(&mut self, event: CommandEvent, _ctx: &mut Context<Self>) -> Result<()> {
        match event {
            CommandEvent::Stdout { key, value } => {
                self.state.set(key, value);
            }
            CommandEvent::Terminated(_) => {}
        }
        Ok(())
    }
}

struct Render;

impl DoSync<Render> for HubApp {
    fn once(&mut self, _: &mut Render) -> Result<Next<Self>> {
        let terminal = self.terminal.get_mut()?;
        terminal.draw(|frame| {
            self.state.render(frame);
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
