use yew::prelude::*;
use yew::worker::*;

use serde::{Deserialize, Serialize};

pub struct Toast {}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Toast(String),
}

impl Agent for Toast {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = String;

    fn create(_link: AgentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) {
        unimplemented!()
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        unimplemented!()
    }
}
