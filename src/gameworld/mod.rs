//Jerome M. St.Martin
//May, 2022

use std::sync::mpsc::{Receiver, Sender};

use super::common::command::Message;

pub struct GameWorld {
    gui_coms: (Receiver<Message>, Sender<Message>),
}

impl GameWorld {
    pub fn new(rx: Receiver<Message>, tx: Sender<Message>) -> Self {
        GameWorld {
            gui_coms: (rx, tx),
        }
    }

    pub fn tick(&mut self) {
        //TODO
    }
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_0() {}
}
