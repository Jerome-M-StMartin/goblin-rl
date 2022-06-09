//Jerome M. St.Martin
//June, 2022

//-----------------------------------------------------------------------------
//---------------------------- ECS Access Point: ------------------------------
//------------------------- for Multithreaded Access --------------------------
//----------------------------- to ECS Storages -------------------------------
//-----------------------------------------------------------------------------

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Condvar};

mod storage_accessor;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessKey {
    Placeholder, //replace with real variant
    //add variants representing all components as components are implemented
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
    //mtx_and_cvar: Arc<MtxAndCvar>,
    mtx: Mutex<Access>,
    cvar: Condvar,
}

impl AccessGuard {
    pub(super) fn new() -> Self {
        AccessGuard {
            mtx: Mutex::new( Access { readers: 0, read_allowed: true, write_allowed: true } ),
            cvar: Condvar::new(),
        }
    }
}

impl Drop for AccessGuard {
    fn drop(&mut self) {
        let mut access = self.mtx.lock().expect("AccessGuard Mutex poisoned before .drop()");

        if !access.write_allowed && !access.read_allowed {
            //This AccessGuard was giving exclusive Write access,
            //so it is now safe to allow any type of access.
            access.write_allowed = true;
            access.read_allowed = true;

        } else if !access.write_allowed && access.read_allowed {
            //This AccessGuard was granding non-exclusive Read access,
            //so the reader count must be decremented.
            
            //POSSIBLE WRITER-STARVATION PROBLEM, but I'm not sure it matters?
            
            access.readers -= 1;
        }

        if access.readers == 0 { 
            //Write access is allowed again, since there is no one with access currently.
            access.write_allowed = true;

            //Due to possible writer-starvation problem:
            //Here, if required, a thread awaiting WRITE access should be notified.
            //This behaviour requires a second Condvar in AccessGuard, specifically
            //used by threads waiting for Write access.
        }

        self.cvar.notify_all();
    }
}
