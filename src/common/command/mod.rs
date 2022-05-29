mod message;

use std::sync::Mutex;

pub use message::Message;

//-----------------------------------------------------
//---------------- COMMAND PATTERN --------------------
//-----------------------------------------------------

/* T should be an enum, e.g.:
pub enum Command {
    Move { dir: Dir },
    GoTo { pos: Point },
    Select,
    Undo,
}
*/

pub trait Commandable<T>
where T: PartialEq + Eq + Copy + Clone,
{
    //Implementors should have CommandQueue & CommandHistory fields.
    fn send(&self, command: T); //store cmd in CommandQueue
    fn execute_one(&self); //execute first cmd in CommandQueue
    fn execute_all(&self); //execute all cmds in CommandQueue
    fn undo(&self) {} //Optional, requires CommandHistory field
}

// Command Storage for lazy processing/execution
#[derive(Debug)]
pub struct CommandQueue<T> {
    queue: Mutex<Vec<T>>,
}

impl<T> CommandQueue<T> {
    pub fn new() -> Self {
        CommandQueue { queue: Mutex::new(Vec::new()), }
    }

    pub fn push(&self, cmd: T) {
        if let Ok(mut queue_guard) = self.queue.lock() {
            queue_guard.push(cmd);
        } else { panic!("Mutex poisoned in command::CommandQueue."); };
    }

    pub fn head(&self) -> Option<T> { //use this for true Queue-like behaviour
        if let Ok(mut queue_guard) = self.queue.lock() {
            if !queue_guard.is_empty() {
                return Some(queue_guard.remove(0)); //O(n), but preserves order
            }
        } else { panic!("Mutex poisoned in command::CommandQueue."); };

        None
    }

    pub fn tail(&self) -> Option<T> {
        if let Ok(mut queue_guard) = self.queue.lock() {
            if !queue_guard.is_empty() {
                return queue_guard.pop();
            }
            
        } else { panic!("Mutex poisoned in command::CommandQueue."); };

        None
    }
    
    pub fn clear(&self) {
        if let Ok(mut queue_guard) = self.queue.lock() {
            queue_guard.clear()
        } else { panic!("Mutex poisoned in command::CommandQueue."); };
    }

    pub fn into_iter(&self) -> CommandQueueIter<T> {
        if let Ok(queue_guard) = self.queue.lock() {
            return CommandQueueIter {
                guard: queue_guard,
            }
        }
        panic!("Mutex poisoned. (command::CommandQueue::iter())");
    }
}

// CommandQueue.iter() creates this struct and uses it to hold a MutexGuard,
// so that the guard can be returned and access to the Vec in CommandQueue is
// exposed via a slice. i.e. CmdQ.iter() -> Vec<T>.iter() -> &[T].
pub struct CommandQueueIter<'a, T> {
    guard: std::sync::MutexGuard<'a, Vec<T>>,
}

impl<'a, 'b: 'a, T: 'a> IntoIterator for &'b CommandQueueIter<'a, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.guard.iter()
    }
}

//Funtionality for undo-ability of Commands
#[derive(Debug)]
pub struct CommandHistory<T> {
    history: Mutex<Vec<T>>,
}

//Only the owning Commandable obj should ever be calling these.
impl<T> CommandHistory<T> {
    pub fn new() -> Self {
        CommandHistory { history: Mutex::new(Vec::new()), }
    }

    pub fn push(&mut self, cmd: T) {
        if let Ok(mut history) = self.history.lock() {
            history.push(cmd);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Ok(mut history) = self.history.lock() {
            return history.pop();
        };

        None
    }
}
