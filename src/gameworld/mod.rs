//Jerome M. St.Martin
//May, 2022

use std::sync::mpsc::{Receiver, SyncSender};

use crate::common::{Message, Ticker};
use crate::error::Gremlin;

mod map;

pub struct GameWorld {
    channel: (Receiver<Message>, SyncSender<Message>),
}

impl GameWorld {
    pub fn new(rx: Receiver<Message>, tx: SyncSender<Message>) -> Self {
        GameWorld {
            channel: (rx, tx),
        }
    }

    pub fn tick(&mut self) -> Result<Ticker, Gremlin> {
        println!("GW thread calling recv()...\r");

        let message = self.channel.0.recv()?;
        if message == Message::Exit { return Ok(Ticker::ExitProgram) };

        println!("{:?}\r", message);
        
        Ok(Ticker::Continue)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_0() {}
}
