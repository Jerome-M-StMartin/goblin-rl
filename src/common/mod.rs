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
        if c.x < square_map_width - 1 {
            return Ok(Coords::new(c.x + 1, c.y))
        }
        Err(Gremlin::OutOfMapBounds)
    }
    pub fn south_of(c: Coords, square_map_width: u16) -> Result<Coords, Gremlin> {
        if c.y < square_map_width - 1 {
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



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_north_of() {
        let c_ok = Coords::new(1u16,1u16);
        let c_err = Coords::new(1u16, 0u16);

        assert_eq!(Coords::north_of(c_ok).unwrap(), Coords::new(1u16, 0u16));
        assert!(Coords::north_of(c_err).is_err());
    }

    #[test]
    fn test_east_of() {
        let c_ok = Coords::new(0u16,0u16);
        let c_ok2 = Coords::new(0u16,1u16);
        let c_err = Coords::new(1u16, 0u16);
        let c_err2 = Coords::new(1u16, 1u16);

        assert_eq!(Coords::east_of(c_ok, 2).unwrap(), Coords::new(1u16, 0u16));
        assert_eq!(Coords::east_of(c_ok2, 2).unwrap(), Coords::new(1u16, 1u16));
        assert!(Coords::east_of(c_err, 2).is_err());
        assert!(Coords::east_of(c_err2, 2).is_err());
    }

    #[test]
    fn test_south_of() {
        let c_ok = Coords::new(0u16,0u16);
        let c_ok2 = Coords::new(1u16,0u16);
        let c_err = Coords::new(0u16, 1u16);
        let c_err2 = Coords::new(1u16, 1u16);

        assert_eq!(Coords::south_of(c_ok, 2).unwrap(), Coords::new(0u16, 1u16));
        assert_eq!(Coords::south_of(c_ok2, 2).unwrap(), Coords::new(1u16, 1u16));
        assert!(Coords::south_of(c_err, 2).is_err());
        assert!(Coords::south_of(c_err2, 2).is_err());
    }

    #[test]
    fn test_west_of() {
        let c_ok = Coords::new(1u16,0u16);
        let c_err = Coords::new(0u16, 0u16);

        assert_eq!(Coords::west_of(c_ok).unwrap(), Coords::new(0u16, 0u16));
        assert!(Coords::west_of(c_err).is_err());
    }
}
