//Jerome M. St.Martin
//June, 2022

//-----------------------------------------------------------------------------
//---------------------------- ECS Access Point: ------------------------------
//------------------------- for Multithreaded Access --------------------------
//----------------------------- to ECS Storages -------------------------------
//-----------------------------------------------------------------------------

use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};

mod storage_access_guard;

pub use storage_access_guard::StorageAccessGuard;

/*
//Marker Structs to hold Component Types for use in AccessKey variants.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct TypeMarker<T> {
    _phantom: PhantomData<T>,
}

impl<T> TypeMarker<T> {
    pub fn new() -> Self {
        TypeMarker {
            _phantom: PhantomData::<T>,
        }
    }
}*/

//Each variant REPRESENTS the type of the Storage which access is being saught for.
//The type of the Storage corresponds to the Type of the Component it stores.
//It is up to me, the programmer, to account for that representation, because
//I cannot for the life of me figure out how to work it into Rust's type system,
//primarily because the AccessGuards live in collections and the collections live
//in a NON-generic struct (ECSAccessPoint). If I explicitly put a generic type on
//the AccessGuard, that means I have to explicitly generically type the collections
//they live in, and that means I have to explicitly generically type the ECSAccessPoint,
//but... that breaks the whole point, which is that there is only one, non-generic,
//ECSAccessPoint in which all AccessGuards live, regardless of the type of storage
//they guard.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AccessKey {
    TestComponent,
    //add variants representing each component as components are implemented
}

pub struct ECSAccessPoint {
    accessors: HashMap<AccessKey, Arc<AccessGuard>>,
    ecs: specs::World,
}

impl ECSAccessPoint {
    pub fn new(ecs: specs::World) -> Self {
        ECSAccessPoint {
            accessors: HashMap::new(),
            ecs,
        }
    }

    pub fn req_access(&mut self, key: AccessKey) -> Arc<AccessGuard> {
        if !self.accessors.contains_key(&key) {
            self.accessors.insert(key, Arc::new(AccessGuard::new()));
        }
        self.accessors.get(&key).unwrap().clone()
    }

    pub fn borrow_ecs(&self) -> &specs::World {
        &self.ecs
    }
}

//These structs must be wrapped in a Mutex.
#[derive(Debug)]
struct Access {
    pub readers: u8,
    pub read_allowed: bool,
    pub write_allowed: bool,
}

// These structs must be wrapped in an Arc<> before being
// passed to the thread(s) which is/are seeking access.
pub struct AccessGuard {
    mtx: Mutex<Access>,
    cvar: Condvar,
    //_phantom: PhantomData<T>,
}

impl AccessGuard {
    pub(super) fn new() -> Self {
        AccessGuard {
            mtx: Mutex::new(Access {
                readers: 0,
                read_allowed: true,
                write_allowed: true,
            }),
            cvar: Condvar::new(),
            //_phantom: PhantomData::<T>,
        }
    }
}

impl Drop for AccessGuard {
    fn drop(&mut self) {
        let mut access = self
            .mtx
            .lock()
            .expect("AccessGuard Mutex poisoned before .drop()");

        if !access.write_allowed && !access.read_allowed {
            //This AccessGuard was giving exclusive Write access,
            //so it is now safe to allow any type of access.
            access.write_allowed = true;
            access.read_allowed = true;
        } else if !access.write_allowed && access.read_allowed {
            //This AccessGuard was granding non-exclusive Read access,
            //so the reader count must be decremented.

            //POSSIBLE WRITER-STARVATION PROBLEM, but I'm not sure it matters
            //in this context? Like... all ECS work that will be done also must
            //be done in-between each user input event. And every aspect of this
            //program's multithreadedness is blocking... so doesn't that
            //guarantee that write access will never be starved? I think yes.

            access.readers -= 1;
        }

        if access.readers == 0 {
            //Write access is allowed again, since there is no one with access currently.
            access.write_allowed = true;

            //Due to possible writer-starvation problem:
            //Here, if required, a thread awaiting WRITE access should be notified.
            //This behaviour requires a second Condvar in AccessGuard, specifically
            //used by threads waiting for Write access.
            //Seems unneccesary at this time, but I'll leave these notes for future me,
            //just in case I am very wrong, which has a definite non-zero probability.
        }

        self.cvar.notify_all();
    }
}
