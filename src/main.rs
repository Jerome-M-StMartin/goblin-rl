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

const MAP_SIZE: u16 = 10;

fn main() {

    /* Control Flow:
     * 1.) The User gives input which is sent from Controller to View.
     * 2.) Based on the User's input and the context of its state, the View either:
     *     2a.) Attempts to mutate its state, in which case the control flow ends.
     *     2b.) Sends a MutateCommand to the Model, in which case, move on to step 3 ->
     * 3-0.) The Model receives the MutateCommand.
     * 3-1.) The Model mutates its state based on the MutateCommand.
     * 3-2.) The Model further mutates its state as required by the previous mutation(s), loop.
     * 3-3.) The Model sends a DeltaNotification to the View.
     * 4-0.) The View receives the DeltaNotification.
     * 4-1.) The View mutates its state based on the DeltaNotification, which notifies the User.
     * 4-2.) The Controller waits for User Input.
     *
     * Each MutateCommand may set off a chain of mutations which only the Game World knows how
     * to handle, which is why the View must wait to receive a DeltaNotification detailing the
     * entire chain, so it (the View) knows how to mutate itself to reflect these changes to
     * the GameWorld, and in turn shows the User the effect if their input.
    */ 

    let (mutate_tx, mutate_rx) = mpsc::sync_channel(1); // View <--> Model Channel A
    let (delta_tx, delta_rx) = mpsc::sync_channel(1); //View <--> Model Channel B
    let (ui_tx, ui_rx) = mpsc::sync_channel(1); //Controller -> View Channel

    /* ---------------------------
     * ---------- MODEL ----------
     * ---------------------------
     */
    // Init & Spawn the GameWorld thread
    let gw_thread = thread::spawn(move || {

        let mut gw = gameworld::GameWorld::new(mutate_rx, delta_tx, MAP_SIZE);

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
    // Init & Spawn the TUI thread
    let tui_thread = thread::spawn(move || {

        let mut tui = tui::TUIState::new(ui_rx, delta_rx, mutate_tx);

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
    let mut gs = ctrlr::MainState::new(gw_thread, tui_thread, ui_tx);

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

