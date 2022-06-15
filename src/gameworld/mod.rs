//Jerome M. St.Martin
//May, 2022

use std::sync::{
    mpsc::{Receiver, SyncSender},
    Arc,
};

//specs lib docs say this should be imported over just World

use crate::common::{DeltaNotification, MutateCommand, Ticker};
use crate::ecs_access_point::{AccessKey, ECSAccessPoint, StorageAccessGuard};
use crate::error::Gremlin;

//ECS Modules
pub mod components;
pub mod resources;
mod systems;

pub struct GameWorld {
    channel: (Receiver<MutateCommand>, SyncSender<DeltaNotification>),
    ecs_ap: Arc<ECSAccessPoint>,
}

impl GameWorld {
    pub fn new(
        map_size: u16, //unused is OK, test-map precon doesn't take a size
        rx: Receiver<MutateCommand>,
        tx: SyncSender<DeltaNotification>,
        ecs_ap: Arc<ECSAccessPoint>,
    ) -> Self {
        GameWorld {
            channel: (rx, tx),
            ecs_ap,
        }
    }

    pub fn tick(&mut self) -> Result<Ticker, Gremlin> {
        //println!("GW thread calling recv()...\r");
        let cmd = self.channel.0.recv()?;
        // println!("{:?}\r", cmd); //FOR TESTING ONLY

        match cmd {
            MutateCommand::Test => {
                //println!("Test Successful! You just hit Enter/Return.\r");
                self.ecs_ap.print_map();
            }
            MutateCommand::Exit => {
                return Ok(Ticker::ExitProgram);
            }
            _ => {}
        };

        Ok(Ticker::Continue)
    }
}
