//Jerome M. St.Martin
//May, 2022

use std::any::Any;
use std::thread::{JoinHandle, Thread};
use std::sync::mpsc::Sender;

use specs::WorldExt; //specs lib docs say this should be imported over just World

use super::user_input::UserInput;
use super::common::Message;

#[derive(Clone, Debug)]
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
    ecs: specs::World,
    game_world: JoinHandle<Thread>, //Game Simulation State
    gui: JoinHandle<Thread>, //GUI State
    gui_tx: Sender<Message>,
    gw_tx: Sender<Message>,
    runstate: RunState,
    user_input: UserInput,
}

impl MainState {
    pub fn new(game_world: JoinHandle<Thread>,
               gui: JoinHandle<Thread>,
               gui_tx: Sender<Message>,
               gw_tx: Sender<Message>) -> MainState {

        MainState {
            ecs: specs::World::new(),
            game_world,
            gui,
            gui_tx,
            gw_tx,
            runstate: RunState::MainMenu,
            user_input: UserInput::new(),
        }
    }

    pub fn tick(&mut self) -> Result<(), super::error::Gremlin> {

        //Has there been any user input?
        self.user_input.tick()?;
        
        //user_input.event guaranteed to be None after this.
        let input_event = self.user_input.take_input_event();

        //println!("{:?}\r", input_event); //For User Input Testing Only
        
        match &self.runstate {
            RunState::AwaitingInput { previous: _prev } => {},
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
    pub fn join_threads(self) -> (Result<Thread, Box<dyn Any + Send>>, Result<Thread, Box<dyn Any + Send>>) {
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
