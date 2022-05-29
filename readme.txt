GoblinRL v0.2 - May, 2022

Author: Jerome M. St.Martin
Language: Rust
What Is: Video Game Engine + Roguelike

This is a terminal-rendering, text-only game engine written for the
purpose of building several games, the first of which will be a
somewhat traditional rogue-like. 
(https://en.wikipedia.org/wiki/Rogue_(video_game))

//--------------------------------------------------------------------
//-------------------- Hiring Manager Information --------------------
//--------------------------------------------------------------------
1.) Design Patterns (implemented from scratch):
    - a flavor of MVC
    - Command Pattern
    - Observer Pattern (many Observers per Subject)
    - [On Its Way] Builder Pattern

2.) Concurrency:
    This software operates over three threads, primarily to provide
    a seamless UX by decoupling game-state processing from GUI
    processing. Performance enhancement is not a driving factor for
    implementing concurrency, as very little of the code-base runs
    in parallel. (https://en.wikipedia.org/wiki/Amdahl%27s_law)

3.) Design Documents:
    - Please see class_diagram.md in the 'mermaid' directory, which
      you can run with a Mermaid VSCode extension.
    - Architecture Notes: [Not Yet Migrated from Old Codebase]
    - Design Notes: [Not Yet Migrated from Old Codebase]
//--------------------------------------------------------------------
//--------------------------------------------------------------------
//--------------------------------------------------------------------

Why I migrated this repo here:
Old codebase was very messy, as it was a learning experience in
both Rust and game development. I think a rebuild may be the faster
route to a finished product, rather than in-place mutation of my
code.
