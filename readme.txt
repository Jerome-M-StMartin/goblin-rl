GoblinRL v0.2
Version Rewrite began May, 2022
Last ReadMe Edit: June 14, 2022

Author: Jerome M. St.Martin
Language: Rust
What Is: Video Game Engine + Roguelike

!--- Work in Progress ---!

This is a terminal-rendering, text-only game engine written for the
purpose of building several games, the first of which will be a
somewhat traditional rogue-like. 
(https://en.wikipedia.org/wiki/Rogue_(video_game))

//--------------------------------------------------------------------
//-------------------- Hiring Manager Information --------------------
//--------------------------------------------------------------------
1.) Implemented Design Patterns:
    - a flavor of multithreaded MVC (Model-View-Controller)
    - Command Pattern (eliminated in favor of threading + channels)
    - Observer Pattern (many Observers per Subject)
    - [In Progress for GameWorld Map] Builder Pattern

2.) Concurrency:
    This software operates over three threads, primarily to provide
    a seamless UX by decoupling game-state processing from UI
    processing. Performance enhancement is not a driving factor for
    implementing concurrency, as very little of the code-base runs
    in parallel. (https://en.wikipedia.org/wiki/Amdahl%27s_law)
    
    See the ECSAccessPoint/ecs_access_point module for an example of
    a blocking control structure providing safe multithreaded access
    by many readers xor exclusive writer over the ECS state. Makes
    use of Condvar and Mutex from the Rust std lib.

3.) Design Documents:
    - Please see class_diagram.md in the 'mermaid' directory, which
      you can run with a Mermaid VSCode extension.
    - Architecture Notes: [Not Yet Public]
    - Design Notes: [Not Yet Public]
//--------------------------------------------------------------------
//--------------------------------------------------------------------
//--------------------------------------------------------------------

Why I migrated this repo here:
Old codebase was very messy, as it was a learning experience in
both Rust and game development. I think a rebuild may be the faster
route to a finished product, rather than in-place mutation of my
code. So far, this is proving to be very, very true.
