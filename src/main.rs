//Jerome M. St.Martin
//May, 2022

use std::sync::{mpsc, Arc};
use std::thread;

use specs::WorldExt;

mod common;
mod controller;
mod ecs_access_point;
mod error;
mod gameworld;
mod tui;
mod user_input;

use ecs_access_point::ECSAccessPoint;
use gameworld::{components, resources};

const MAP_SIZE: u16 = 10;

fn main() {

    // ECS Initialization
    let mut ecs_world: specs::World = WorldExt::new();
    resources::insert_all_resources(&mut ecs_world);
    components::register_all_components(&mut ecs_world);

    let ecs_ap = Arc::new(ECSAccessPoint::new(ecs_world));
    let gw_ecs_ap = ecs_ap.clone();
    let tui_ecs_ap = ecs_ap.clone();

    //Channel endpoint names are derived from the enums they send/recv.
    let (mutate_tx, mutate_rx) = mpsc::sync_channel(1); // View --> Model
    let (delta_tx, delta_rx) = mpsc::sync_channel(1); // Model --> View
    let (ui_tx, ui_rx) = mpsc::sync_channel(1); // Controller -> View

    /* ---------------------------
     * ---------- MODEL ----------
     * ---------------------------
     */
    // Init & Spawn the GameWorld thread
    let gw_thread = thread::spawn(move || {
        let mut gw = gameworld::GameWorld::new(MAP_SIZE, mutate_rx, delta_tx, gw_ecs_ap);

        loop {
            match gw.tick() {
                Ok(ticker) => match ticker {
                    common::Ticker::ExitProgram => {
                        break;
                    }
                    common::Ticker::Continue => {}
                },
                Err(e) => {
                    println!("{}", e);
                }
            };
        }
    });

    /* ---------------------------
     * ---------- VIEW -----------
     * ---------------------------
     */
    // Init & Spawn the TUI thread
    let tui_thread = thread::spawn(move || {
        let mut tui = tui::TUIState::new(ui_rx, delta_rx, mutate_tx, tui_ecs_ap);

        loop {
            match tui.tick() {
                Ok(ticker) => match ticker {
                    common::Ticker::ExitProgram => {
                        break;
                    }
                    common::Ticker::Continue => {}
                },
                Err(e) => {
                    println!("{}", e);
                }
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
    let mut gs = controller::MainState::new(gw_thread, tui_thread, ui_tx, ecs_ap);

    loop {
        match gs.tick() {
            Ok(ticker) => match ticker {
                common::Ticker::ExitProgram => {
                    break;
                }
                common::Ticker::Continue => {}
            },
            Err(e) => {
                println!("{}", e);
            }
        };
    }

    //----------- End & Clean Up -----------
    println!("Joining...\r");
    let (_, _) = gs.join_threads();
    println!("Exiting...\r");
    crossterm::terminal::disable_raw_mode().unwrap();
    std::process::exit(0);
}
