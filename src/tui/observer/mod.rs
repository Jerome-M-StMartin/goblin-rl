//Jerome M. St.Martin
//12/18/2020

//OBSERVER DESIGN PATTERN
//Many Observers per Subject/Observable

use std::any::Any;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub struct IdGenerator {
    used_ids: Mutex<HashSet<usize>>,
}
impl IdGenerator {
    pub fn new() -> Self {
        IdGenerator {
            used_ids: Mutex::new(HashSet::new()),
        }
    }

    //Guaranteed to return a unique usize for this session.
    pub fn generate_observer_id(&self) -> usize {
        let mut new_id: usize = rand::random();
        if let Ok(mut used_ids) = self.used_ids.lock() {
            while used_ids.contains(&new_id) {
                new_id = rand::random();
            }
            used_ids.insert(new_id);
        }
        new_id
    }
}

pub trait Observer: Send + Sync {
    type Subject: Observable;
    //Each implementor of Observer must store a unique observer id,
    //generated by Observable::ID_GENERATOR.generate_observer_id().
    fn update(&self);

    fn id(&self) -> usize;
    fn become_focus(&self) { /*optional*/ }
    fn name(&self) -> &str { "Placeholder" } //for debugging
}

pub trait Observable : Send + Sync {
    const ID_GENERATOR: IdGenerator;
    type Observers: IntoIterator;

    fn notify_observers(&self); //<-implement lazy removal of dropped observers in here.
    fn notify_focus(&self);
    fn add_observer<T: 'static + Observer>(&self, to_add: &Arc<T>) -> usize;
    fn as_any(&self) -> &dyn Any; // Implement by returning self, e.g.: { self }
}

/* EXAMPLE Observable Trait IMPLEMENTATION:
impl Observable for MyStruct {
    fn notify(&self, to_notify: &Vec<Box<dyn Observer>>) {
        for observer in to_notify {
            observer.update();
        }
    }

    fn add_observer(to_add: Box<dyn Observer>, to_notify: &mut Vec<Box<dyn Observer>>) {
        to_notify.push(to_add);
    }

    fn rm_observer(&self, to_remove: &Box<dyn Observer>, to_notify: &mut Vec<Box<dyn Observer>>) {
        let mut to_remove_idx = 0;
        for observer in to_notify.iter() {
            if observer.id() == to_remove.id() {
                break;
            }
            to_remove_idx += 1;
        }

        //swap_remove() used over remove() for O(1) runtime.
        //Currently, the order of Observers in this vec doesn't matter,
        //if this changes remove() will have to be used instead.
        to_notify.swap_remove(to_remove_idx);
    }
}*/
