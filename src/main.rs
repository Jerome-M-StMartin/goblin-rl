//Jerome M. St.Martin
//May, 2022

use std::thread;
use std::sync::mpsc;

mod common;
mod error;
mod gameworld;
mod gui;
mod main_state;

fn main() {

    let (gui_tx, gameworld_rx) = mpsc::channel();
    let (gameworld_tx, gui_rx) = mpsc::channel();

    /* ---------------------------
     * ---------- MODEL ----------
     * ---------------------------
     */
    // Init & Spawn the GameWorld thread
    let gw_thread = thread::spawn(move || {

        let mut gw = gameworld::GameWorld::new(gameworld_rx, gameworld_tx);

        loop {
            gw.tick();
        }

    });

    /* ---------------------------
     * ---------- VIEW -----------
     * ---------------------------
     */
    // Init & Spawn the GUI thread
    let gui_thread = thread::spawn(move || {
        
        let mut gui = gui::GUIState::new(gui_rx, gui_tx);

        loop {
            gui.tick();
        }

    });

    /* ---------------------------
     * ------- CONTROLLER --------
     * ---------------------------
     */
    // Store JoinHandles on gui & gameworld threads in MainState struct
    let mut ms = main_state::MainState::new(gw_thread, gui_thread);
    let mut running = Result::Ok(());

    while running.is_ok() { 
        running = ms.tick();
    }

    //----------- End & Clean Up -----------
    let (_, _) = ms.join_all();
}

