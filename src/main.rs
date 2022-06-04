//Jerome M. St.Martin
//May, 2022

use std::thread;
use std::sync::mpsc;

mod common;
mod error;
mod gameworld;
mod tui;
mod ctrlr;
mod user_input;

fn main() {

    let (tui_tx, tui_rx) = mpsc::sync_channel(1); //message flow: tui -> gameworld
    let (gameworld_tx, gameworld_rx) = mpsc::sync_channel(1); //message flow: gameworld -> tui
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
            match gw.tick() {
                Ok(ticker) => {
                    match ticker {
                        common::Ticker::ExitProgram => { break; },
                        common::Ticker::Continue => {},
                    }
                },
                Err(e) => {
                    println!("{}", e);
                },   
            };
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
            match tui.tick() {
                Ok(ticker) => {
                    match ticker {
                        common::Ticker::ExitProgram => { break; },
                        common::Ticker::Continue => {},
                    }
                },
                Err(e) => {
                    println!("{}", e);
                },   
            };
        }

    });

    /* ---------------------------
     * ------- CONTROLLER --------
     * ---------------------------
     */

    //Enable raw input mode, so all user input is captured immediately, byte-by-byte, as-is.
    crossterm::terminal::enable_raw_mode().unwrap(); //panics on failure, which is desired

    // Store JoinHandles on tui & gameworld threads in GameState struct
    let mut gs = ctrlr::MainState::new(gw_thread, tui_thread, ctrlr_to_tui, ctrlr_to_gw);

    loop { 
        match gs.tick() {
            Ok(ticker) => {
                match ticker {
                    common::Ticker::ExitProgram => { break; },
                    common::Ticker::Continue => {},
                }
            },
            Err(e) => {
                println!("{}", e);
            },
        };
    }

    //----------- End & Clean Up -----------
    println!("Joining...\r");
    let (_, _) = gs.join_threads();
    println!("Exiting...\r");
    crossterm::terminal::disable_raw_mode().unwrap();
    std::process::exit(0);
}

