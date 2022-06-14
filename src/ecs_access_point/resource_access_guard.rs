//Jerome M. St.Martin
//June 13, 2022

//-----------------------------------------------------------------------------
//---------------- Controls Access to Individual ECS Resources ----------------
//-----------------------------------------------------------------------------

use specs::{
    prelude::Resource,
    shred::{Fetch, FetchMut},
}; 

use super::{Access, AccessGuard};

pub trait ResourceAccessGuard<'a> {
    fn read_resource<T: Resource>(&self, ecs: &'a specs::World) -> Fetch<'a, T>;
    fn write_resource<T: Resource>(&self, ecs: &'a specs::World) -> FetchMut<'a, T>;
}

impl<'a> ResourceAccessGuard<'a> for AccessGuard {
    fn read_resource<T: Resource>(&self, ecs: &'a specs::World) -> Fetch<'a, T> {
        const READ_ERR_MSG: &str = "ResourceAccessGuard mutex poisoned before read.";
        
        let mut access: std::sync::MutexGuard<'_, Access> = self
            .cvar
            .wait_while(self.mtx.lock().expect(READ_ERR_MSG), |acc: &mut Access| {
                !acc.read_allowed
            })
        .expect(READ_ERR_MSG);

        access.write_allowed = false;
        access.read_allowed = true;
        access.readers += 1;

        ecs.fetch()
    }

    fn write_resource<T: Resource>(&self, ecs: &'a specs::World) -> FetchMut<'a, T> {
        const WRITE_ERR_MSG: &str = "ResourceAccessGuard mutex poisoned before write.";

        let mut access: std::sync::MutexGuard<'_, Access> = self
            .cvar
            .wait_while(self.mtx.lock().expect(WRITE_ERR_MSG), |acc: &mut Access| {
                !acc.read_allowed
            })
        .expect(WRITE_ERR_MSG);

        access.read_allowed = false;
        access.write_allowed = false;

        ecs.fetch_mut()
    }
}
