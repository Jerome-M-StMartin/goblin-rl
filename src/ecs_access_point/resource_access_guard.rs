//Jerome M. St.Martin
//June 13, 2022

//-----------------------------------------------------------------------------
//---------------- Controls Access to Individual ECS Resources ----------------
//-----------------------------------------------------------------------------

use specs::{
    prelude::Resource,
    shred::{Fetch, FetchMut},
}; 

use super::{AccessorState, AccessGuard};

pub trait ResourceAccessGuard<'a> {
    fn read_resource<T: Resource>(&self, ecs: &'a specs::World) -> Fetch<'a, T>;
    fn write_resource<T: Resource>(&self, ecs: &'a specs::World) -> FetchMut<'a, T>;
}

impl<'a> ResourceAccessGuard<'a> for AccessGuard {
    fn read_resource<T: Resource>(&self, ecs: &'a specs::World) -> Fetch<'a, T> {
        const READ_ERR_MSG: &str = "ResourceAccessGuard mutex poisoned before read.";
        
        let mut accessor_state: std::sync::MutexGuard<'_, AccessorState> = self
            .cvar
            .wait_while(self.mtx.lock().expect(READ_ERR_MSG), |acc_state: &mut AccessorState| {
                !acc_state.read_allowed
            })
        .expect(READ_ERR_MSG);

        accessor_state.write_allowed = false;
        accessor_state.read_allowed = true;
        accessor_state.readers += 1;

        ecs.fetch()
    }

    fn write_resource<T: Resource>(&self, ecs: &'a specs::World) -> FetchMut<'a, T> {
        const WRITE_ERR_MSG: &str = "ResourceAccessGuard mutex poisoned before write.";

        let mut accessor_state: std::sync::MutexGuard<'_, AccessorState> = self
            .cvar
            .wait_while(self.mtx.lock().expect(WRITE_ERR_MSG), |acc_state: &mut AccessorState| {
                !acc_state.read_allowed
            })
        .expect(WRITE_ERR_MSG);

        accessor_state.read_allowed = false;
        accessor_state.write_allowed = false;

        ecs.fetch_mut()
    }
}
