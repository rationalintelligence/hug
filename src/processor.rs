use crb::kit::actor::{Actor, ActorSession};

pub struct Processor {}

impl Actor for Processor {
    type Context = ActorSession<Self>;
}
