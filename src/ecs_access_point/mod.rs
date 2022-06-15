//Jerome M. St.Martin
//June, 2022

//-----------------------------------------------------------------------------
//---------------------------- ECS Access Point: ------------------------------
//------------------------- for Multithreaded Access --------------------------
//----------------------------- to ECS Storages -------------------------------
//-----------------------------------------------------------------------------

use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};

use specs::{Component, Entity, World};

mod storage_access_guard;
mod resource_access_guard;

pub use storage_access_guard::StorageAccessGuard;
pub use resource_access_guard::ResourceAccessGuard;

//FOR TESTING ----------
use crate::gameworld::resources::map::Map;
use crate::common::Coords;
//----------------------    

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AccessKey { //add variants representing each Component or Resource as needed
    //Resources
    Map,
    Player,

    //Components
    Hostile,
    Position,
}

pub struct ECSAccessPoint {
    accessors: Mutex<HashMap<AccessKey, Arc<Accessor>>>,
    ecs: World,
}

impl ECSAccessPoint {
    pub fn new(ecs: specs::World) -> Self {
        ECSAccessPoint {
            accessors: Mutex::new(HashMap::new()),
            ecs,
        }
    }

    //FOR TESTING ONLY
    fn helper(map: &Map) {
        let mut idx = 0;
        for _ in 0..map.size {
            print!("\n\r");
            for _ in 0..map.size {
                let glyph: char;
                if map.walls[idx as usize] { glyph = '#'; } else { glyph = '.'; };
                print!("{}", glyph);
                idx += 1;
            }
        }
        print!("\n\r");
    }

    //FOR TESTING ONLY
    pub fn print_map(&self) {
        let access_guard = self.req_access(AccessKey::Map);
        let map = access_guard.read_resource::<Map>(&self.ecs);

        Self::helper(&map);

        for i in 0..map.size {
            print!("\n\r");
            for j in 0..map.size {
                if let Ok(wall_glyph) = map.prettify_wall(&map.walls, Coords::new(j, i)) {
                    print!("{}", wall_glyph);
                } else {
                    print!(".");
                }
            }
        }
        print!("\n\r");
    }

    pub fn insert_component<T: Component>(
        &self,
        key: AccessKey,
        c: T,
        e: Entity,
    ) -> Result<Option<T>, specs::error::Error> {
        self.req_access(key)
            .write_storage::<T>(&self.ecs)
            .insert(e, c)
    }

    fn req_access(&self, key: AccessKey) -> AccessGuard {
        let mut accessors = self
            .accessors
            .lock()
            .expect("Mutex found to be poisoned during ecs_ap.req_access()");
        
        let accessor_arc = accessors.entry(key) //If AccessGuard found, skip next line
            .or_insert(Arc::new(Accessor::new())) //else insert new AccessGuard
            .clone();

        AccessGuard::new(accessor_arc)
    }
}

//These structs must be wrapped in a Mutex.
#[derive(Debug)]
//struct Access {
struct AccessorState {
    pub readers: u8,
    pub read_allowed: bool,
    pub write_allowed: bool,
}

//new abstraction
pub struct Accessor {
    mtx: Mutex<AccessorState>,
    cvar: Condvar,
}

impl Accessor {
    pub(super) fn new() -> Self {
        Accessor {
            mtx: Mutex::new(AccessorState {
                readers: 0,
                read_allowed: true,
                write_allowed: true,
            }),
            cvar: Condvar::new(),
        }
    }
}

pub struct AccessGuard(Arc<Accessor>);

impl AccessGuard {
    pub(super) fn new(accessor: Arc<Accessor>) -> Self {
        AccessGuard(accessor.clone())
    }
}

impl std::ops::Deref for AccessGuard {
    type Target = Accessor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for AccessGuard {
    fn drop(&mut self) {

        let mut access = self
            .mtx
            .lock()
            .expect("AccessGuard Mutex poisoned before .drop()");

        match (access.write_allowed, access.read_allowed) {
            (false, false) => {
                //This AccessGuard was giving exclusive Write access,
                //so it is now safe to allow any type of access.
                access.write_allowed = true;
                access.read_allowed = true;
            },

            (false, true) => {
                //This AccessGuard was granding non-exclusive Read access,
                //so the reader count must be decremented.

                //POSSIBLE WRITER-STARVATION PROBLEM, but I'm not sure it matters
                //in this context? Like... all ECS work that will be done also must
                //be done in-between each user input event. And every aspect of this
                //program's multithreadedness is blocking... so doesn't that
                //guarantee that write access will never be starved? I think yes.
                access.readers -= 1;

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
            },

            (w, r) => {
                panic!("This Condvar configuration should not be possible: ({}, {})", w, r)
            },
        }


        self.cvar.notify_all();
    }
}
