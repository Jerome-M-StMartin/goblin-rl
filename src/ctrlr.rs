//Jerome M. St.Martin
//May, 2022

use std::any::Any;
use std::thread::{JoinHandle, Thread};
use std::sync::mpsc::SyncSender;

use specs::WorldExt; //specs lib docs say this should be imported over just World

use crate::user_input::UserInput;
use crate::common::{Message, Ticker};
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
    game_world: JoinHandle<()>, //Game Simulation State
    tui: JoinHandle<()>, //GUI State
    tui_tx: SyncSender<Message>,
    gw_tx: SyncSender<Message>,
    runstate: RunState,
}

impl MainState {
    pub fn new(game_world: JoinHandle<()>,
               tui: JoinHandle<()>,
               tui_tx: SyncSender<Message>,
               gw_tx: SyncSender<Message>) -> MainState {

        MainState {
            ecs: specs::World::new(),
            game_world,
            tui,
            tui_tx,
            gw_tx,
            runstate: RunState::MainMenu,
        }
    }

    pub fn tick(&mut self) -> Result<Ticker, Gremlin> {

        //Has there been any user input?
        println!("blocking read in ctrlr\r");
        let user_input: Message = UserInput::blocking_read()?;

        //FOR TESTING ONLY - cause the game to stop running correctly
        if user_input == Message::Exit {
            println!("Exit branch reached!\r");
            //Cause TUI thread to finish
            self.tui_tx.send(Message::Exit)?;

            //Cause GameWorld thread to finish
            self.gw_tx.send(Message::Exit)?;
            
            //Finish this thread
            return Ok(Ticker::ExitProgram)
        }; 
      
        println!("send() called in ctrlr\r");
        self.tui_tx.send(user_input)?;

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

        Ok(Ticker::Continue)
    }
    
    //Used to stop the two main threads upon Game Over or Game Close
    pub fn join_threads(self) -> (Result<(), Box<dyn Any + Send>>, Result<(), Box<dyn Any + Send>>) {
        //Returns two results.
        let gw = match self.game_world.join() {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        };
        let tui = match self.tui.join() {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        };

        (gw, tui)
    }
}
