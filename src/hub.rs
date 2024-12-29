use anyhow::Result;
use async_trait::async_trait;
use crb::kit::actor::{Actor, ActorSession, OnRequest, Request, Standalone};

pub struct Hub {}

impl Hub {
    pub fn new() -> Self {
        Self {}
    }
}

impl Standalone for Hub {}

#[async_trait]
impl Actor for Hub {
    type Context = ActorSession<Self>;
}

// ADD FLOW

pub struct AddFlow {
    pub class: String,
}

impl Request for AddFlow {
    type Response = ();
}

#[async_trait]
impl OnRequest<AddFlow> for Hub {
    async fn on_request(&mut self, request: AddFlow, ctx: &mut Self::Context) -> Result<()> {
        todo!()
    }
}
