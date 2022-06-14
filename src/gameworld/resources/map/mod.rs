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
 * Each tile is represented by an Index, which is used as a universal key into
 * the fields of the Map struct which describe aspects of the map, and of each
 * tile in it.
 */

// Type Aliasing
use usize as Index;

pub struct Map {
    pub dirty_flag: bool,
    pub size: u16,
    pub player_spawnpoint: Index,
    pub walls: Vec<bool>, //Must be initialized to have size^2 elements.
    pub blocked: Vec<bool>, //Must be initialized to have size^2 elements.
    pub tile_contents: HashMap<Index, [Entity; 8]>,
}

impl Map {

    pub fn builder() -> map_builder::MapBuilder {
        map_builder::MapBuilder::new()
    }

    //This should probably never be used directly,
    //use the Builder Pattern functionality instead.
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

    ///Pure version also available
    pub fn coords_to_idx(&self, coords: Coords) -> Result<Index, Gremlin> {
        if coords.x < self.size && coords.y < self.size {
            return Ok( (coords.x + coords.y * self.size) as Index )
        }
        
        Err( Gremlin::InvalidInput )
    }

    ///Pure version also available
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
    
    /// Creates a 4-bit bitmask representing the orthogonally adjacent
    /// tiles which are also walls, in order to determine which line-glyph
    /// to draw for the passed-in wall tile.
    pub fn prettify_wall(square_map_width: u16,
                     walls_vec: &Vec<bool>,
                     wall_coords: Coords) -> Result<char, Gremlin> {

        let wall_idx = Map::pure_coords_to_idx(square_map_width, wall_coords)?;
        if !walls_vec[wall_idx] {
            return Err(Gremlin::InvalidInput)
        };

        let mut north: bool = false;
        let mut east: bool = false;
        let mut south: bool = false;
        let mut west: bool = false;

        if let Ok(north_coords) = Coords::north_of(wall_coords) {
            let idx = Map::pure_coords_to_idx(square_map_width, north_coords)?;
            north = walls_vec[idx];
        }

        if let Ok(east_coords) = Coords::east_of(wall_coords, square_map_width) {
            let idx = Map::pure_coords_to_idx(square_map_width, east_coords)?;
            east = walls_vec[idx];
        }

        if let Ok(south_coords) = Coords::south_of(wall_coords, square_map_width) {
            let idx = Map::pure_coords_to_idx(square_map_width, south_coords)?;
            south = walls_vec[idx];
        }

        if let Ok(west_coords) = Coords::west_of(wall_coords) {
            let idx = Map::pure_coords_to_idx(square_map_width, west_coords)?;
            west = walls_vec[idx];
        }

        let glyph = match (north, east, south, west) {
            //Four Orthogonally Adjacent
            (true, true, true, true) => { '╋' },

            //Three Adjacent
            (true, true, true, false) => { '┣' },
            (true, true, false, true) => { '┻' },
            (true, false, true, true) => { '┫' },
            (false, true, true, true) => { '┳' },

            //Two Adjacent
            (true, true, false, false) => { '┗' },
            (true, false, false, true) => { '┛' },
            (false, false, true ,true) => { '┓' },
            (false, true, true, false) => { '┏' },
            (true, false, true, false) => { '┃' },
            (false, true, false, true) => { '━' },

            //One Adjacent
            (true, false, false, false) => { '╹' },
            (false, true, false, false) => { '╺' },
            (false, false, true, false) => { '╻' },
            (false, false, false, true) => { '╸' },

            //None Adjacent
            (false, false, false, false) => { '■' },
        };

        return Ok(glyph)
    }

    ///Method version also available
    pub fn pure_coords_to_idx(square_map_width: u16, coords: Coords) -> Result<Index, Gremlin> {
        if coords.x < square_map_width && coords.y < square_map_width {
            return Ok( (coords.x + coords.y * square_map_width) as Index )
        }
        
        Err( Gremlin::InvalidInput )
    }

    ///Method version also available
    pub fn pure_idx_to_coords<T: Into<u32>>(square_map_width: u16, idx: T) -> Result<Coords, Gremlin> {
        let idx = idx.into();
        let size = square_map_width as u32;

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
