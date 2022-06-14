//Jerome M. St.Martin
//May, 2022

use crate::error::Gremlin;

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

    pub fn north_of(c: Coords) -> Result<Coords, Gremlin> {
        if c.y > 0 {
            return Ok(Coords::new(c.x, c.y - 1))
        }
        Err(Gremlin::OutOfMapBounds)
    }
    pub fn east_of(c: Coords, square_map_width: u16) -> Result<Coords, Gremlin> {
        if c.x < square_map_width {
            return Ok(Coords::new(c.x + 1, c.y))
        }
        Err(Gremlin::OutOfMapBounds)
    }
    pub fn south_of(c: Coords, square_map_width: u16) -> Result<Coords, Gremlin> {
        if c.y < square_map_width {
            return Ok(Coords::new(c.x, c.y + 1))
        }
        Err(Gremlin::OutOfMapBounds)
    }
    pub fn west_of(c: Coords) -> Result<Coords, Gremlin> {
        if c.x > 0 {
            return Ok(Coords::new(c.x - 1, c.y))
        }
        Err(Gremlin::OutOfMapBounds)
    }
}
