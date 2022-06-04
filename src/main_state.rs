//Jerome M. St.Martin
//May, 2022

use std::any::Any;
use std::thread::{JoinHandle, Thread};
use std::sync::mpsc::SyncSender;

use specs::WorldExt; //specs lib docs say this should be imported over just World

use crate::user_input::UserInput;
use crate::common::Message;
use crate::error::Gremlin;

//-------------------------------------------
//--------------- CONTROLLER ----------------
//----------------- of MVC ------------------
//-------------------------------------------
//
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
    gui_tx: SyncSender<Message>,
    gw_tx: SyncSender<Message>,
    runstate: RunState,
}

impl MainState {
    pub fn new(game_world: JoinHandle<Thread>,
               gui: JoinHandle<Thread>,
               gui_tx: SyncSender<Message>,
               gw_tx: SyncSender<Message>) -> MainState {

        MainState {
            ecs: specs::World::new(),
            game_world,
            gui,
            gui_tx,
            gw_tx,
            runstate: RunState::MainMenu,
        }
    }

    pub fn tick(&mut self) -> Result<(), Gremlin> {

        //Has there been any user input?
        let user_input: Message = UserInput::blocking_read()?;

        //FOR TESTING ONLY - basically testing if I can cause the game to stop running correctly
        //which partially tests user input. Need to further test user_input by sending messages
        //recieved by TUI thread back to the main thread to be printed via {:?}
        if user_input == Message::Cancel {
            //TODO: Cause TUI thread to finish
            //TODO: Cause GameWorld thread to finish
            
            //return any error to force main thread to call join_threads then exit()
            return Err(Gremlin::InvalidInput)
        }; 
       
        self.gui_tx.send(user_input)?;

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
