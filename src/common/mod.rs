//Jerome M. St.Martin
//May, 2022

/*
 * PURPOSE:
 * Structs and modules shared between both GUI and GameWorld threads.
 */

//pub mod command; This is useless with channels; how should it be, now?

pub enum Message {
    Temp,
}

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
pub struct Coords { pub x: u8, pub y: u8 }

impl Coords {
    pub fn new<T: Into<u8>>(x: T, y: T) -> Self {
        Coords {x: x.into(), y: y.into()}
    }
}
