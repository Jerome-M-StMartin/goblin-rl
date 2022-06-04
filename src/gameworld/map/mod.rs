//Jerome M. St.Martin
//June 2, 2022


mod tile;

use tile::Tile;
use crate::common::Coords;
use crate::error::Gremlin;



//-----------------------------------------------------------------------------
//------------------------------ Game Map -------------------------------------
//------------------------------- Builder -------------------------------------
//-----------------------------------------------------------------------------

use usize as Index; //Type Alias

struct Map {
    size: u8,
    blocked: Vec<bool> ,
    tiles: Vec<Tile>,
}

impl Map {
    pub(super) fn new<T: Into<u8>>(size: T) -> Self
    {
        Map {
            size: size.into(),
            blocked: vec![false],
            tiles: vec![Tile::default()],
        }
    }

    pub(super) fn coords_to_idx(&self, coords: Coords) -> Result<Index, Gremlin> {
        if coords.x < self.size && coords.y < self.size {
            return Ok( (coords.x + coords.y * self.size) as Index )
        }
        
        Err( Gremlin::InvalidInput )
    }

    pub(super) fn idx_to_coords<T: Into<u8>>(&self, idx: T) -> Result<Coords, Gremlin> {
        let idx = idx.into();

        if idx < self.size.pow(2) {
            let x = idx % self.size;
            let y = idx / self.size;
            return Ok( Coords::new(x, y) )
        }
        
        Err( Gremlin::InvalidInput )
    }
}



#[cfg(test)]
mod test {
    
    use super::*;

    #[test]
    fn test_coords_to_idx() {
        let map = Map::new(10);

        assert!(map.coords_to_idx(Coords::new(0, 1)).unwrap() == 10 as usize);
        assert!(map.coords_to_idx(Coords::new(1, 1)).unwrap() == 11 as usize);
        assert!(map.coords_to_idx(Coords::new(1, 0)).unwrap() == 1 as usize);
        assert!(map.coords_to_idx(Coords::new(9, 0)).unwrap() == 9 as usize);
        assert!(map.coords_to_idx(Coords::new(0, 9)).unwrap() == 90 as usize);
        assert!(map.coords_to_idx(Coords::new(9, 9)).unwrap() == 99 as usize);

        assert!(map.coords_to_idx(Coords::new(10, 10)).is_err());
    }

    #[test]
    fn test_idx_to_coords() {
        let map = Map::new(10);
        
        assert!(map.idx_to_coords(0).unwrap() == Coords::new(0, 0));
        assert!(map.idx_to_coords(9).unwrap() == Coords::new(9, 0));
        assert!(map.idx_to_coords(10).unwrap() == Coords::new(0, 1));
        assert!(map.idx_to_coords(18).unwrap() == Coords::new(8, 1));
        assert!(map.idx_to_coords(19).unwrap() == Coords::new(9, 1));
        assert!(map.idx_to_coords(99).unwrap() == Coords::new(9, 9));

        assert!(map.idx_to_coords(100).is_err());
    }
}
