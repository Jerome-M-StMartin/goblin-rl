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
mod ecs_access_point;

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
     *
     * Problem: Multiple Sources of Truth == High Maintenance Reqs.
     *      This forces the View to keep its own stateful representation of things
     *      which are already statefully represented in the Model, meaning that
     *      any mutation to one MUST result in an identical mutation to the other.
     *      This will be annoying, tedious, and error-prone; or so it seems currently.
     *      The benefit to this way of doing things is that there is no shared state
     *      between the View and Model threads, which is... pretty nice.
     * 
     * Possible Solution: SPECS Storages as Shared State
     *      Assuming Storages can be placed in RwLocks (preferred to Mutex, though at first
     *      I thought the opposite; explanation below), doing so allows both View and Model
     *      threads to hold an Arc<RwLock<Storage>>. Though, this seems like the wrong
     *      way to interact with Storages ... I don't want to have to hold a Vec of all
     *      possible Storages in each thread... I need to test this out somehow, or
     *      otherwise determine a way to acces the ECS from multiple threads.
     *
     *      Why RwLock is preferred:
     *      My initial assumption was that View would always Read, and Model would always
     *      Write. This is false. View will sometimes Write to instert "Intent" Components,
     *      and Model will sometimes only need Read access (because many systems will simply
     *      require Read access to make a sufficient Join over the desired Entities). Thus,
     *      a RwLock over each Storage seems ideal. Though, the problem remains that I'm not
     *      sure how to place a RwLock over each individual storage...
     *
     *      Solution Continued: I think I found a way.
     *      From the SPECS docs:
     *      "It is strictly disallowed to fetch both a ReadStorage and a
     *      WriteStorage of the same component. Because Specs uses interior mutability for
     *      its resources, we can’t check this at compile time. If you try to do this, you
     *      will get a panic."
     *
     *      Therefor, I need to create a struct, say... an ECSHandle, which acts as a guard
     *      over the ECS types which allow storage access (ReadStorage, WriteStorage, etc).
     *      Both View and Model threads will hold an Arc<ECSHandle>. The ECSHandle will
     *      handle the logic for guaranteeing that only one Write or only Reads occur at
     *      any one time for any specific Storage. Right now, I'm thinking it'll just create
     *      unique IDs (usizes probably) for each storage type (dynamically, as access is
     *      requested), which it places into BeingWritten or BeingRead collections.
     *      Simple queries on these two collections will allow it to either block any thread
     *      requesting storage access or allow that thread access. This seems like a REALLY
     *      fun project; I'll have to learn how to properly block a thread without spinning.
     *      Maybe thread::yield? I think I recently read in Jon G.'s book that yielding is
     *      not ideal, so I'll have to look into that.
     *
     *      Solution Continued (again): Performant alternative to yield = Condvar + Mutex.
     *      Here's the plan! (Very happy to figure this out before I have to be away from
     *      my dev environment for ~24hrs. :] Not sarcasm! When I get back it'll be clear
     *      exactly what my next step forward is to be, and that feels AMAZING, as other
     *      software devs will appreciate, I am sure. Anyway...
     *      The solution is: Impl a pure public fn on Arc<ECSHandle>, blocking_fetch(storage),
     *      which DYNAMICALLY creates a (Mutex<bool>, Condvar) tuple to guard access to
     *      that specific storage. Actually, instead of blocking_fetch(...) I'll do
     *      write_fetch(...) and read_fetch(...), but I digress.
     *      The Mutex/Condvar tuple will be accessed/stored via Rust's Entry API, such that
     *      there will only ever be 0..1 tuples instantiated for any given Read or Write
     *      to any given Storage. The collection of currently existing Write tuples will
     *      always be checked first, even for read_fetch(). If that's all-clear, then the
     *      Read tuple collection will be checked. In this way, an existing Read will
     *      allow other Reads without blocking, but any existing Write will have guaranteed
     *      exclusive access to a single specific storage for the duration of the Write access.
     *
     *      Woo! I hope this concept implements as well as it was designed. :] :) :}
     *
     *      Also, changing "ECSHandle" to ECSAccessPoint, which is more sensical imo.
    */      

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

        let mut gw = gameworld::GameWorld::new(MAP_SIZE, mutate_rx, delta_tx);

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

