//Jerome M. St.Martin
//May, 2022

use std::sync::mpsc::{Receiver, SyncSender};

use super::common::Message;

mod map;

pub struct GameWorld {
    gui_coms: (Receiver<Message>, SyncSender<Message>),
}

impl GameWorld {
    pub fn new(rx: Receiver<Message>, tx: SyncSender<Message>) -> Self {
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
