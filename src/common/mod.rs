//Jerome M. St.Martin
//May, 2022

/*
 * PURPOSE:
 * Structs and modules shared between both GUI and GameWorld threads.
 */

pub mod command;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Point { x: i32, y: i32 }
