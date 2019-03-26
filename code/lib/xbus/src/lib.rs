extern crate actix;
extern crate futures;
use actix::*;

pub struct CommandBus;

impl Actor for CommandBus {
    type Context = Context<Self>;
}
