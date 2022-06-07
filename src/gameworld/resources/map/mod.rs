//Jerome M. St.Martin
//June 2, 2022

use std::collections::HashMap;

use specs::Entity;

use crate::common::Coords;
use crate::error::Gremlin;

pub mod precon;
mod map_builder;

//-----------------------------------------------------------------------------
//--------------------------- Game World Map ----------------------------------
//-----------------------------------------------------------------------------

/* Abstract Tiles:
 * Each tile is represented by an Index,
 * which is used as a universal key into
 * the fields of the Map struct which
 * describe aspects of the map, and of
 * each tile in it.
 */

// Type Aliasing
use usize as Index;

pub struct Map {
    dirty_flag: bool,
    size: u16,
    player_spawnpoint: Index,
    walls: Vec<bool>, //Must be initialized to have size^2 elements.
    blocked: Vec<bool>, //Must be initialized to have size^2 elements.
    tile_contents: HashMap<Index, [Entity; 8]>,
}

impl Map {

    pub fn builder() -> map_builder::MapBuilder {
        map_builder::MapBuilder::new()
    }

    pub fn new<T: Into<u16> + Copy>(size: T) -> Self
    {
        Map {
            dirty_flag: false,
            size: size.into(),
            player_spawnpoint: 11,
            walls: vec![false; size.into().pow(2) as usize],
            blocked: vec![false; size.into().pow(2) as usize],
            tile_contents: HashMap::new(),
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty_flag
    }

    pub fn clean(&mut self) {
        self.dirty_flag = false;
    }

    pub fn coords_to_idx(&self, coords: Coords) -> Result<Index, Gremlin> {
        if coords.x < self.size && coords.y < self.size {
            return Ok( (coords.x + coords.y * self.size) as Index )
        }
        
        Err( Gremlin::InvalidInput )
    }

    pub fn idx_to_coords<T: Into<u32>>(&self, idx: T) -> Result<Coords, Gremlin> {
        let idx = idx.into();
        let size = self.size as u32;

        if idx < size.pow(2) {
            let x = (idx % size) as u16;
            let y = (idx / size) as u16;
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
