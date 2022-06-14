//Jerome M. St.Martin
//June 6, 2022

//-----------------------------------------------------------------------------
//--------------------------- ECS Resource Module -----------------------------
//-----------------------------------------------------------------------------

pub(crate) mod map;

pub(crate) fn insert_all_resources(ecs: &mut specs::World) {
    ecs.insert(generate_map());
}

fn generate_map() -> map::Map {
    map::Map::builder()
        .with_precon_layout(map::precon::empty_10x10())
        .build()
}
