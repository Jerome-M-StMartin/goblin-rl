//Jerome M. St.Martin
//June, 2022

//-----------------------------------------------------------------------------
//---------------------------- ECS Access Point: ------------------------------
//------------------------- for Multithreaded Access --------------------------
//----------------------------- to ECS Storages -------------------------------
//-----------------------------------------------------------------------------

use std::collections::HashMap;
use std::sync::{Mutex, Condvar};

type AccessGranted = bool;

pub struct ECSAccessPoint { //What to use as keys???
    active_reads: HashMap<(/*???*/), StorageGuard>,
    active_writes: HashMap<(/*???*/), StorageGuard>,
    ecs: specs::World,
}

#[derive(Debug)]
struct StorageGuard { //Do these also guard Resource access???
    mtx: Mutex<AccessGranted>,
    cvar: Condvar,
}

impl ECSAccessPoint {
    pub fn new(ecs: specs::World) -> Self {
        ECSAccessPoint {
            active_reads: HashMap::new(),
            active_writes: HashMap::new(),
            ecs,
        }
    }
}

impl StorageGuard {
    pub fn new() -> Self {
        StorageGuard {
            mtx: Mutex::new(false),
            cvar: Condvar::new(),
        }
    }

    pub fn mtx(&mut self) -> &mut Mutex<bool> {
        &mut self.mtx
    }

    pub fn cvar(&self) -> &Condvar {
        &self.cvar
    }
}
