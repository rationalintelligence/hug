use anyhow::Result;
use async_trait::async_trait;
use crb::kit::actor::{Actor, ActorSession, OnRequest, Request, Standalone};

pub struct AddFlow {}

impl Request for AddFlow {
    type Response = ();
}

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

#[async_trait]
impl OnRequest<AddFlow> for Hub {
    async fn on_request(
        &mut self,
        request: AddFlow,
        ctx: &mut Self::Context,
    ) -> Result<T::Response> {
        todo!()
    }
}
