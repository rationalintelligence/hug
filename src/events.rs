use crate::app::HubApp;
use anyhow::Result;
use crb::agent::{Address, Agent, AgentSession, DoSync, Next, ToAddress};
use crossterm::event;

pub struct EventsDrainer {
    app: Address<HubApp>,
}

impl EventsDrainer {
    pub fn new(app: impl ToAddress<HubApp>) -> Self {
        Self {
            app: app.to_address(),
        }
    }
}

impl Agent for EventsDrainer {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_sync(())
    }
}

impl DoSync for EventsDrainer {
    fn repeat(&mut self, _: &mut ()) -> Result<Option<Next<Self>>> {
        let event = event::read()?;
        self.app.event(event)?;
        Ok(None)
    }
}
