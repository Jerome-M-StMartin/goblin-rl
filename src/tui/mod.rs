//Jerome M. St.Martin
//May, 2022

use std::sync::mpsc::{Receiver, SyncSender};

use specs::Entity;

use crate::common::{DeltaNotification, InputEvent, InsertionData, MutateCommand, Ticker};
use crate::ecs_access_point::AccessKey;
use crate::error::Gremlin;

mod observer;

pub struct TUIState {
    ctrlr_channel: Receiver<InputEvent>,
    model_channel: (Receiver<DeltaNotification>, SyncSender<MutateCommand>),
}

impl TUIState {
    pub fn new(
        ctrlr_rx: Receiver<InputEvent>,
        model_rx: Receiver<DeltaNotification>,
        model_tx: SyncSender<MutateCommand>,
    ) -> Self {
        TUIState {
            ctrlr_channel: (ctrlr_rx),
            model_channel: (model_rx, model_tx),
        }
    }

    pub fn tick(&mut self) -> Result<Ticker, Gremlin> {
        //println!("TUI thread calling recv()...\r"); // FOR TESTING ONLY
        let message = self.ctrlr_channel.recv()?;
        //println!("{:?}\r", message); // FOR TESTING ONLY

        //Process Message
        match message {
            InputEvent::Confirm => {
                //Testing ECS Access Point
                self.model_channel.1.send(MutateCommand::Test)?;
            }
            InputEvent::Exit => {
                self.pre_exit(&self.model_channel.1)?;
                return Ok(Ticker::ExitProgram);
            }
            _ => {}
        }

        Ok(Ticker::Continue)
    }

    fn pre_exit(&self, gw_tx: &SyncSender<MutateCommand>) -> Result<Ticker, Gremlin> {
        //Tell GameWorld thread to finish
        gw_tx.send(MutateCommand::Exit)?;

        Ok(Ticker::ExitProgram)
    }
}

#[cfg(test)]
mod tests {

    //use super::*;

    #[test]
    fn test_0() {}
}
