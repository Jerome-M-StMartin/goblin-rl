//Jerome M. St.Martin
//June 15, 2022

//-----------------------------------------------------------------------------
//-------------------------- Initializers & Spawners --------------------------
//------------------------------ for ECS Entities -----------------------------
//-----------------------------------------------------------------------------

use specs::{Component, Entity};
use super::components::*;

pub(crate) fn build_player_entity(ecs: &mut specs::World, spawn_at: Coords) -> Entity {
    ecs
        .create_entity()
        .with(Player {})
        .with(Position(spawn_at))
        .build()
}
