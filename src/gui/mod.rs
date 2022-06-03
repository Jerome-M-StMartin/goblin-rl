//Jerome M. St.Martin
//May, 2022

use std::sync::mpsc::{Receiver, Sender};

use super::common::Message;

mod observer;

pub struct GUIState {
    gameworld_coms: (Receiver<Message>, Sender<Message>),
}

impl GUIState {
    pub fn new(rx: Receiver<Message>, tx: Sender<Message>) -> Self {
        GUIState {
            gameworld_coms: (rx, tx),
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
