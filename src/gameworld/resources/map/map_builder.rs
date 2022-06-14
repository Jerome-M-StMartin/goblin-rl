//Jerome M. St.Martin
//June 6, 2022

//-----------------------------------------------------------------------------
//---------------------------- Map Builder Pattern ----------------------------
//-----------------------------------------------------------------------------

use super::Map;
use super::precon::*;

//Fields are instructions to build a Map struct.
pub struct MapBuilder {
    size: Option<u16>,
    layout: Option<&'static str>,
}

impl MapBuilder {
    pub fn new() -> Self {
        MapBuilder {
            size: None,
            layout: None,
        }
    }

    pub fn with_precon_layout(mut self, precon: PreCon) -> Self {
        self.layout = Some(precon.layout);
        self.size = Some(precon.size);
        self
    }

    pub fn with_procgen_layout(mut self) -> Self {
        //TODO
        self
    }

    pub fn build(self) -> Map {
        let mut map = Map::new(self.size.unwrap());
        
        for (idx, c) in str_to_no_whitespace_chars(self.layout.unwrap()).enumerate() {
            match c {
                '#' => {
                    map.walls[idx] = true;
                    map.blocked[idx] = true;
                },
                '.' => { /* empty tile */ },
                '@' => { 
                    map.player_spawnpoint = idx;
                    map.blocked[idx] = true;
                },
                _ => { println!("c: {}\r", c); },
            }
        }

        map
    }
}

fn str_to_no_whitespace_chars(s: &'static str) -> impl Iterator<Item = char> {
    s.chars().filter(|c| *c != '\r' && *c != '\n' && *c != ' ')
}
