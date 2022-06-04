//Jerome M. St.Martin
//May, 2022

use std::sync::mpsc::{Receiver, SyncSender};

use crate::common::Message;
use crate::error::Gremlin;

mod observer;

pub struct GUIState {
    channel: (Receiver<Message>, SyncSender<Message>),
}

impl GUIState {
    pub fn new(rx: Receiver<Message>, tx: SyncSender<Message>) -> Self {
        GUIState {
            channel: (rx, tx),
        }
    }


    pub fn tick(&mut self) -> Result<(), Gremlin> {
        
        let message = self.channel.0.try_recv()?;

        println!("{:?}\r", message);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    //use super::*;

    #[test]
    fn test_0() {}
}
