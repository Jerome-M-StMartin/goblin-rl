//Jerome M. St.Martin
//June 8, 2022

//-----------------------------------------------------------------------------
//---------------- Controls Access to Individual ECS Storages -----------------
//------------------------- & Returns StorageGuards ---------------------------
//-----------------------------------------------------------------------------

use std::sync::{Arc, Mutex, Condvar, MutexGuard};
use std::marker::PhantomData;

use specs::{Component, WorldExt};

use super::StorageGuard;

type MTX_AND_CVAR = (Mutex<Access>, Condvar);

const READ_ERR_MSG: &str = "StorageAccessor mutex poisoned before read.";
const WRITE_ERR_MSG: &str = "StorageAccessor mutex poisoned before write.";

#[derive(Debug)]
struct Access {
    pub read_allowed: bool,
    pub write_allowed: bool,
}

#[derive(Debug)]
pub(super) struct StorageAccessor<T: Component> {
    mtx_and_cvar: Arc<MTX_AND_CVAR>,
    phantom: PhantomData<T>,
}

impl<'a, T: Component> StorageAccessor<T> {
    pub(super) fn new() -> Self {
        StorageAccessor {
            mtx_and_cvar: Arc::new(
                              (Mutex::new( Access { read_allowed: true, write_allowed: true } ),
                               Condvar::new()),
                          ),
            phantom: PhantomData,
        }
    }

    fn get_access(&self) -> Arc<MTX_AND_CVAR> {
        self.mtx_and_cvar.clone()
    }

    //TODO: I think the ecs access as a borrow doesn't work. Fix. Need sync wrapper?
    fn read_storage(ecs: &'a specs::World, mtx_and_cvar: Arc<MTX_AND_CVAR>) -> StorageGuard<'a, T> {
        let m_c = mtx_and_cvar;

        //While read access is NOT allowed, wait until the calling thread is notified on the condvar.
        //Once the condvar (cvar) is notified, the calling thread is awoken,
        //the lock for the arg.0 mutex (mtx) is acquired,
        //and execution of this function continues.
        let mut access = m_c.1.wait_while(m_c.0.lock().expect(READ_ERR_MSG),
                            |access| { !access.read_allowed })
                            .expect(READ_ERR_MSG);

        access.read_allowed = true;
        access.write_allowed = false;
        StorageGuard::new_readable(ecs.read_component())
    }

    //TODO: I think the ecs access as a borrow doesn't work. Fix. Need sync wrapper?
    fn write_storage(ecs: &'a specs::World, mtx_and_cvar: Arc<MTX_AND_CVAR>) -> StorageGuard<'a, T> {
        let m_c = mtx_and_cvar;

        /*While write access is NOT allowed, wait until the calling thread is notified on the
         * condvar. Once the condvar is notified, the calling thread is awoken,
         * the lock for the arg.0 mutex is acquired,
         * and the execution of this function continues. */
        let mut access = m_c.1.wait_while(m_c.0.lock().expect(WRITE_ERR_MSG),
                                         |access| { !access.write_allowed })
                                         .expect(WRITE_ERR_MSG);

        access.read_allowed = false;
        access.write_allowed = false;

        StorageGuard::new_writeable(ecs.write_component())
    }

}
