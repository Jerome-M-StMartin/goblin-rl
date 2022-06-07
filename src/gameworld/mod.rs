//Jerome M. St.Martin
//May, 2022

use std::sync::mpsc::{Receiver, SyncSender};

use specs::WorldExt; //specs lib docs say this should be imported over just World

use crate::common::{MutateCommand, DeltaNotification, Ticker};
use crate::error::Gremlin;

//ECS Modules
mod components;
mod resources;
mod systems;

pub struct GameWorld {
    channel: (Receiver<MutateCommand>, SyncSender<DeltaNotification>),
    ecs: specs::World,
}

impl GameWorld {
    pub fn new(map_size: u16, 
               rx: Receiver<MutateCommand>,
               tx: SyncSender<DeltaNotification>) -> Self {
        
        //---Map Initialization---
        let map = resources::map::Map::builder()
            .with_precon_layout(resources::map::precon::empty_10x10())
            .build();

        //---ECS Initialization---
        let mut ecs_world: specs::World = WorldExt::new();
        // Insert all resources:
        ecs_world.insert(map); //The Game World map resource
        components::register_all_components(&mut ecs_world); //self-explanatory

        //---Construct the obj---
        GameWorld {
            channel: (rx, tx),
            ecs: ecs_world,
        }
    }

    pub fn tick(&mut self) -> Result<Ticker, Gremlin> {

        //println!("GW thread calling recv()...\r");
        let cmd = self.channel.0.recv()?;
        if cmd == MutateCommand::Exit { return Ok(Ticker::ExitProgram) };
       // println!("{:?}\r", cmd); //FOR TESTING ONLY

        //TODO: Process MutateCommand
        
        Ok(Ticker::Continue)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_0() {}
}
