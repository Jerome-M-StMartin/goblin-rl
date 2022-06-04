//Jerome M. St.Martin
//May, 2022

use std::thread;
use std::sync::mpsc;

mod common;
mod error;
mod gameworld;
mod tui;
mod main_state;
mod user_input;

fn main() {

    let (tui_tx, gameworld_rx) = mpsc::sync_channel(0); //message flow: tui -> gameworld
    let (gameworld_tx, tui_rx) = mpsc::sync_channel(0); //message flow: gameworld -> tui
    let ctrlr_to_tui = tui_tx.clone(); //message flow: controller -> tui
    let ctrlr_to_gw = gameworld_tx.clone(); //message flow: controller -> gameworld

    /* ---------------------------
     * ---------- MODEL ----------
     * ---------------------------
     */
    // Init & Spawn the GameWorld thread
    let gw_thread = thread::spawn(move || {
        
        let mut gw = gameworld::GameWorld::new(gameworld_rx, gameworld_tx.clone());

        loop {
            gw.tick();
        }

    });

    /* ---------------------------
     * ---------- VIEW -----------
     * ---------------------------
     */
    // Init & Spawn the GUI thread
    let tui_thread = thread::spawn(move || {
        
        let mut tui = tui::GUIState::new(tui_rx, tui_tx.clone());

        loop {
            tui.tick();
        }

    });

    /* ---------------------------
     * ------- CONTROLLER --------
     * ---------------------------
     */
    
    //Enable raw input mode, so all user input is captured immediately, byte-by-byte, as-is.
    crossterm::terminal::enable_raw_mode().unwrap(); //panics on failure, which is desired

    // Store JoinHandles on tui & gameworld threads in MainState struct
    let mut ms = main_state::MainState::new(gw_thread, tui_thread, ctrlr_to_tui, ctrlr_to_gw);
    let mut running = Result::Ok(());

    while running.is_ok() { 
        running = ms.tick();
    }

    //----------- End & Clean Up -----------
    println!("Joining...");
    let (_, _) = ms.join_threads();
    println!("Exiting...");
    std::process::exit(0);
}

