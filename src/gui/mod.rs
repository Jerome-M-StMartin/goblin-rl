//Jerome M. St.Martin
//May, 2022

use std::sync::mpsc::{Receiver, Sender};

use super::common::command::Message;

mod observer;
mod user_input;

pub struct GUIState {
    gameworld_coms: (Receiver<Message>, Sender<Message>),
    user_input: user_input::UserInput,
}

impl GUIState {
    pub fn new(rx: Receiver<Message>, tx: Sender<Message>) -> Self {
        GUIState {
            gameworld_coms: (rx, tx),
            user_input: user_input::UserInput::new(),
        }
    }


    pub fn tick(&mut self) {
        //TODO
    }
}

#[cfg(test)]
mod tests {

    //use super::*;

    #[test]
    fn test_0() {}
}
