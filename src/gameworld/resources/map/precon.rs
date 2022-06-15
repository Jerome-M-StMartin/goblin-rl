//Jerome M. St.Martin
//June 6, 2022

//-----------------------------------------------------------------------------
//-------------------------- Hand-Made/PreCon Maps ----------------------------
//-----------------------------------------------------------------------------

pub struct PreCon {
    pub size: u16, //One side of the square map.
    pub layout: &'static str,
}

pub fn empty_10x10() -> PreCon {
    let map_str = "
    ##########
    #........#
    #........#
    #........#
    #...@....#
    #........#
    #........#
    #........#
    #........#
    ##########
    ";

    PreCon {
        size: 10,
        layout: map_str,
    }
}

pub fn test_3x3() -> PreCon {
    let map_str = "
    ###
    ###
    ###
    ";

    PreCon {
        size: 3,
        layout: map_str,
    }
}
