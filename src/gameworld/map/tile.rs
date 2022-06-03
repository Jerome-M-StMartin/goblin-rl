//Jerome M. St.Martin
//June 2, 2022

use specs::Entity;

#[derive(Copy, Clone, Debug)]
pub(super) struct Tile {
    contents: [Option<Entity>; 8],
}

impl Tile {
    pub(super) fn new() -> Self {
        Tile {
            contents: [None; 8],
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            contents: [None; 8],
        }
    }
}
