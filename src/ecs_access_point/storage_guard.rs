//Jerome M. St.Martin
//June 8, 2022

//-----------------------------------------------------------------------------
//---------------------- Return Type for Storage Accessor ---------------------
//-----------------------------------------------------------------------------

use specs::{ReadStorage, WriteStorage};

pub(super) struct StorageGuard<'a, T: specs::Component> { //Do these also guard Resource access???
    read_storage: Option<ReadStorage<'a, T>>,
    write_storage: Option<WriteStorage<'a, T>>,
}

impl<'a, T: specs::Component> StorageGuard<'a, T> {
    pub(super) fn new_readable(read_storage: ReadStorage<'a, T>) -> Self {
        StorageGuard {
            read_storage: Some(read_storage),
            write_storage: None,
        }
    }

    pub(super) fn new_writeable(write_storage: WriteStorage<'a, T>) -> Self {
        StorageGuard {
            read_storage: None,
            write_storage: Some(write_storage),
        }
    }
}

impl<'a, T: specs::Component> Drop for StorageGuard<'a, T> {
    fn drop(&mut self) {/*
        let mut access_allowed = self.mtx.lock().expect("StorageGuard Mutex poisoned before .drop()");
        *access_allowed = true;
        self.cvar.notify_all();*/
    }
}
