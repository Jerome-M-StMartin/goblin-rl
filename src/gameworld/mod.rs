//Jerome M. St.Martin
//May, 2022

use std::sync::mpsc::{Receiver, SyncSender};

use crate::common::{MutateCommand, DeltaNotification, Ticker};
use crate::error::Gremlin;

mod map;

pub struct GameWorld {
    channel: (Receiver<MutateCommand>, SyncSender<DeltaNotification>),
    map: map::Map,
}

impl GameWorld {
    pub fn new(rx: Receiver<MutateCommand>,
               tx: SyncSender<DeltaNotification>,
               map_size: u16) -> Self {

        GameWorld {
            channel: (rx, tx),
            map: map::Map::new(map_size),
        }
    }

    pub fn tick(&mut self) -> Result<Ticker, Gremlin> {

        //println!("GW thread calling recv()...\r");
        let message = self.channel.0.recv()?;
        if message == MutateCommand::Exit { return Ok(Ticker::ExitProgram) };
       // println!("{:?}\r", message); //FOR TESTING ONLY

        //TODO: Process Message
        
        self.map_delta_check();

        Ok(Ticker::Continue)
    }

    //Check the dirty flag of self.map, if there's been a delta then
    //send a Message to the TUI notifying of the change, including
    //the data necessary for the TUI to mutate its representation
    //of the map (a Vec of Glyphs, probably).
    fn map_delta_check(&mut self) {
        if self.map.is_dirty() {
            //TODO: Send Message::DirtyMap(DeltaData) to TUI
            self.map.clean();
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_0() {}
}
