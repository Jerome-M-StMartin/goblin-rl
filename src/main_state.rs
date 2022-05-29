//Jerome M. St.Martin
//May, 2022

use std::any::Any;
use std::thread::{JoinHandle, Thread};

use super::error::Gremlin;

pub enum RunState {
    AwaitingInput{previous: Option<Box<RunState>>},
    GameOver,
    GameWorld,
    GUI,
    MainMenu,
    MapGeneration,
    NextLevel,
    PreRun,
}

pub struct MainState {
    game_world: JoinHandle<Thread>, //Game Simulation State
    gui: JoinHandle<Thread>, //GUI State
    runstate: RunState,
}

impl MainState {
    pub fn new(game_world: JoinHandle<Thread>, gui: JoinHandle<Thread>) -> MainState {
        MainState {
            game_world,
            gui,
            runstate: RunState::MainMenu,
        }
    }

    pub fn tick(&mut self) -> Result<(), super::error::Gremlin> {
        
        match &self.runstate {
            RunState::AwaitingInput { previous: _prev } => {
                self.runstate = RunState::AwaitingInput { previous: None };
            },
            RunState::GameOver => {},
            RunState::GameWorld => {},
            RunState::GUI => {},
            RunState::MainMenu => {},
            RunState::MapGeneration => {},
            RunState::NextLevel => {},
            RunState::PreRun => {},
        }

        Ok(())
    }
    
    //Used to stop the two main threads upon Game Over or Game Close
    pub fn join_all(self) -> (Result<Thread, Box<dyn Any + Send>>, Result<Thread, Box<dyn Any + Send>>) {
        //Returns two results.
        let gw = match self.game_world.join() {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        };
        let gui = match self.gui.join() {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        };

        (gw, gui)
    }
}
