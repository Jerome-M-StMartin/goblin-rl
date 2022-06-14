//Jerome M. St.Martin
//May, 2022

use std::any::Any;
use std::sync::{mpsc::SyncSender, Arc};
use std::thread::JoinHandle;

use crate::common::{InputEvent, Ticker};
use crate::ecs_access_point::ECSAccessPoint;
use crate::error::Gremlin;
use crate::user_input::UserInput;

//-------------------------------------------
//--------------- CONTROLLER ----------------
//----------------- of MVC ------------------
//-------------------------------------------

type ResTuple = (
    Result<(), Box<dyn Any + Send>>,
    Result<(), Box<dyn Any + Send>>,
);

#[derive(Clone, Debug)]
pub enum RunState {
    AwaitingInput { previous: Box<RunState> },
    GameOver,
    GameWorld,
    Tui,
    MainMenu,
    MapGeneration,
    NextLevel,
    PreRun,
}

pub struct MainState {
    game_world: JoinHandle<()>, //Game Simulation State
    tui: JoinHandle<()>,        //GUI State
    tui_tx: SyncSender<InputEvent>,
    ecs_ap: Arc<ECSAccessPoint>,
    runstate: RunState,
}

impl MainState {
    pub fn new(
        game_world: JoinHandle<()>,
        tui: JoinHandle<()>,
        tui_tx: SyncSender<InputEvent>,
        ecs_ap: Arc<ECSAccessPoint>,
    ) -> MainState {
        MainState {
            game_world,
            tui,
            tui_tx,
            ecs_ap,
            runstate: RunState::MainMenu,
        }
    }

    pub fn tick(&mut self) -> Result<Ticker, Gremlin> {
        let user_input: InputEvent = UserInput::blocking_read()?;

        if user_input == InputEvent::Exit {
            //Gracefully Exit Program
            MainState::pre_exit(&self.tui_tx)?;
            return Ok(Ticker::ExitProgram);
        };

        //Pass user input through to TUI thread
        self.tui_tx.send(user_input)?;

        match &self.runstate {
            RunState::AwaitingInput { previous: _prev } => {}
            RunState::GameOver => {}
            RunState::GameWorld => {}
            RunState::Tui => {}
            RunState::MainMenu => {}
            RunState::MapGeneration => {}
            RunState::NextLevel => {}
            RunState::PreRun => {}
        }

        Ok(Ticker::Continue)
    }

    //Used to stop the two main threads upon Game Over or Game Close
    pub(crate) fn join_threads(self) -> ResTuple {
        //Returns two results.
        (self.game_world.join(), self.tui.join())
    }

    fn pre_exit(tui_tx: &SyncSender<InputEvent>) -> Result<Ticker, Gremlin> {
        //Tell TUI thread to finish
        tui_tx.send(InputEvent::Exit)?;

        Ok(Ticker::ExitProgram)
    }
}
