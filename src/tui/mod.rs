//Jerome M. St.Martin
//May, 2022

use std::sync::mpsc::{Receiver, SyncSender};

use crate::common::{Message, Ticker};
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


    pub fn tick(&mut self) -> Result<Ticker, Gremlin> {
        println!("TUI thread calling recv()...\r");
        let message = self.channel.0.recv()?;

        if message == Message::Exit { return Ok(Ticker::ExitProgram) } ;

        println!("{:?}\r", message);
        
        Ok(Ticker::Continue)
    }
}

#[cfg(test)]
mod tests {

    //use super::*;

    #[test]
    fn test_0() {}
}
