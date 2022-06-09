//Jerome M. St.Martin
//June 8, 2022

//-----------------------------------------------------------------------------
//---------------- Controls Access to Individual ECS Storages -----------------
//-----------------------------------------------------------------------------

//use std::marker::PhantomData;

use specs::{Component, WorldExt, prelude::{ReadStorage, WriteStorage}};

use super::{Access, AccessGuard};

//type MtxAndCvar = (Mutex<Access>, Condvar);

pub(super) trait StorageAccessGuard<'a, T: Component> {
    //type T: 'a + Component; //T impls Component trait
    
    fn read_storage(&self, ecs: &'a specs::World) -> ReadStorage<'a, T>;
    fn write_storage(&self, ecs: &'a specs::World) -> WriteStorage<'a, T>;
}

impl<'a, T: 'a + Component> StorageAccessGuard<'a, T> for AccessGuard {
    //TODO: I think the ecs access as a borrow doesn't work. Fix. Need sync wrapper?
    fn read_storage(&self, ecs: &'a specs::World) -> ReadStorage<'a, T> {
        const READ_ERR_MSG: &str = "StorageAccessGuard mutex poisoned before read.";

        //While read access is NOT allowed, wait until the calling thread is notified on the condvar.
        //Once the condvar (cvar) is notified, the calling thread is awoken,
        //the lock for the mutex (mtx) is acquired, and execution of this function continues.
        let mut access: std::sync::MutexGuard<'_, Access> =
            self.cvar
            .wait_while(self.mtx.lock().expect(READ_ERR_MSG), |acc: &mut Access| { !acc.read_allowed })
            .expect(READ_ERR_MSG);

        access.read_allowed = true;
        access.write_allowed = false;
        access.readers += 1;

        ecs.read_component()
    }

    //TODO: I think the ecs access as a borrow doesn't work. Fix. Need sync wrapper?
    fn write_storage(&self, ecs: &'a specs::World) -> WriteStorage<'a, T> {
        const WRITE_ERR_MSG: &str = "StorageAccessGuard mutex poisoned before write.";

        /*While write access is NOT allowed, wait until the calling thread is notified on the
         * condvar. Once the condvar is notified, the calling thread is awoken,
         * the lock for the mutex is acquired, and the execution of this function continues.*/
        let mut access: std::sync::MutexGuard<'_, Access> =
            self.cvar
            .wait_while(self.mtx.lock().expect(WRITE_ERR_MSG), |acc: &mut Access| { !acc.write_allowed })
            .expect(WRITE_ERR_MSG);

        access.read_allowed = false;
        access.write_allowed = false;

        ecs.write_component()
    }
}
