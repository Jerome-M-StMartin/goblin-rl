//Jerome M. St.Martin
//June, 2022

//-----------------------------------------------------------------------------
//---------------------------- ECS Access Point: ------------------------------
//------------------------- for Multithreaded Access --------------------------
//----------------------------- to ECS Storages -------------------------------
//-----------------------------------------------------------------------------

use std::collections::HashSet;

use specs::Component;

mod storage_accessor;
mod storage_guard;

use storage_guard::StorageGuard;

pub struct ECSAccessPoint<'a, C: Component> { //dyn specs::Component
    active_reads: HashSet<StorageGuard<'a, C>>,
    active_writes: HashSet<StorageGuard<'a, C>>,
    ecs: specs::World,
}

impl<'a, C: Component> ECSAccessPoint<'a, C> {
    pub fn new(ecs: specs::World) -> Self {
        ECSAccessPoint {
            active_reads: HashSet::new(),
            active_writes: HashSet::new(),
            ecs,
        }
    }
}
