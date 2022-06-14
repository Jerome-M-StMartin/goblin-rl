//Jerome M. St.Martin
//May, 2022

mod transmittables;

pub use transmittables::*;

///Used as the inner Ok() type for the various .tick() methods' returned Results.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Ticker {
    ExitProgram,
    Continue,
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

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Coords {
    pub x: u16,
    pub y: u16,
}

impl Coords {
    pub fn new<T: Into<u16>>(x: T, y: T) -> Self {
        Coords {
            x: x.into(),
            y: y.into(),
        }
    }
}
