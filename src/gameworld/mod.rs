//Jerome M. St.Martin
//May, 2022

use std::sync::mpsc::{Receiver, SyncSender};

//specs lib docs say this should be imported over just World
use specs::{Component, Entity, WorldExt};

use crate::common::{DeltaNotification, MutateCommand, Ticker};
use crate::ecs_access_point::{AccessKey, ECSAccessPoint, StorageAccessGuard};
use crate::error::Gremlin;

//ECS Modules
pub mod components;
mod resources;
mod systems;

pub struct GameWorld {
    channel: (Receiver<MutateCommand>, SyncSender<DeltaNotification>),
    ecs_ap: ECSAccessPoint,
}

impl GameWorld {
    pub fn new(
        map_size: u16, //unused is OK, test-map precon doesn't take a size
        rx: Receiver<MutateCommand>,
        tx: SyncSender<DeltaNotification>,
    ) -> Self {
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
            ecs_ap: ECSAccessPoint::new(ecs_world),
        }
    }

    pub fn tick(&mut self) -> Result<Ticker, Gremlin> {
        //println!("GW thread calling recv()...\r");
        let cmd = self.channel.0.recv()?;
        // println!("{:?}\r", cmd); //FOR TESTING ONLY

        match cmd {
            MutateCommand::Test => {
                println!("Test Successful! You just hit Enter/Return.");
            }
            MutateCommand::Exit => {
                return Ok(Ticker::ExitProgram);
            }
            _ => {}
        };

        Ok(Ticker::Continue)
    }

    /* THIS MUST BE MOVED TO CRATE::COMMON
    // This fn may block when it calls .write_storage().
    fn insert_component<T: Component>(&mut self, key: AccessKey, c: T, e: Entity) {
        let access_guard = self.ecs_ap.req_access(key);

        // Because ECSAccessPoint is not generically typed, the type that is
        // implementing the Component trait must be defined here. AccessKey
        // variant names correspond to the type of Storage they 'unlock', so
        // the write_storage fn is typed according to the passed-in key.
        let storage = match key {
            AccessKey::TestComponent => access_guard.write_storage::<T>(self.ecs_ap.borrow_ecs()),
        };
        storage.insert(e, c);
    }*/
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_0() {}
}
