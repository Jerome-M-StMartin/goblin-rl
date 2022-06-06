//Jerome M. St.Martin
//June 2, 2022

use std::collections::HashMap;

use specs::Entity;

use super::super::common::Coords;
use super::super::error::Gremlin;

//-----------------------------------------------------------------------------
//------------------------------ Game Map -------------------------------------
//------------------------------- Builder -------------------------------------
//-----------------------------------------------------------------------------

/* Abstract Tiles:
 * Each tile is represented by an Index,
 * which is used as a universal key into
 * the fields of the Map struct which
 * comprise the features of each tile.
 */

use usize as Index; //Type Alias

pub(super) struct Map {
    dirty_flag: bool,
    size: u16,
    walls: Vec<bool>,
    blocked: Vec<bool>, //Walkable/NonWalkable
    tile_contents: HashMap<Index, [Entity; 8]>,
}

impl Map {
    pub(super) fn new<T: Into<u16>>(size: T) -> Self
    {
        Map {
            dirty_flag: false,
            size: size.into(),
            walls: vec![false],
            blocked: vec![false],
            tile_contents: HashMap::new(),
        }
    }

    pub(super) fn is_dirty(&self) -> bool {
        self.dirty_flag
    }

    pub(super) fn clean(&mut self) {
        self.dirty_flag = false;
    }

    pub(super) fn coords_to_idx(&self, coords: Coords) -> Result<Index, Gremlin> {
        if coords.x < self.size && coords.y < self.size {
            return Ok( (coords.x + coords.y * self.size) as Index )
        }
        
        Err( Gremlin::InvalidInput )
    }

    pub(super) fn idx_to_coords<T: Into<u16>>(&self, idx: T) -> Result<Coords, Gremlin> {
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
        let map = Map::new(10u16);

        assert!(map.coords_to_idx(Coords::new(0u16, 1u16)).unwrap() == 10 as usize);
        assert!(map.coords_to_idx(Coords::new(1u16, 1u16)).unwrap() == 11 as usize);
        assert!(map.coords_to_idx(Coords::new(1u16, 0u16)).unwrap() == 1 as usize);
        assert!(map.coords_to_idx(Coords::new(9u16, 0u16)).unwrap() == 9 as usize);
        assert!(map.coords_to_idx(Coords::new(0u16, 9u16)).unwrap() == 90 as usize);
        assert!(map.coords_to_idx(Coords::new(9u16, 9u16)).unwrap() == 99 as usize);

        assert!(map.coords_to_idx(Coords::new(10u16, 10u16)).is_err());
    }

    #[test]
    fn test_idx_to_coords() {
        let map = Map::new(10u16);
        
        assert!(map.idx_to_coords(0u16).unwrap() == Coords::new(0u16, 0u16));
        assert!(map.idx_to_coords(9u16).unwrap() == Coords::new(9u16, 0u16));
        assert!(map.idx_to_coords(10u16).unwrap() == Coords::new(0u16, 1u16));
        assert!(map.idx_to_coords(18u16).unwrap() == Coords::new(8u16, 1u16));
        assert!(map.idx_to_coords(19u16).unwrap() == Coords::new(9u16, 1u16));
        assert!(map.idx_to_coords(99u16).unwrap() == Coords::new(9u16, 9u16));

        assert!(map.idx_to_coords(100u16).is_err());
    }
}
