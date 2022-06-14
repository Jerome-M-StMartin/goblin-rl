//Jerome M. St.Martin
//June 6, 2022

//-----------------------------------------------------------------------------
//--------------------------- ECS Component Module ----------------------------
//-----------------------------------------------------------------------------

use specs::prelude::*;
use specs_derive::Component;

use crate::common::Coords;

pub(crate) fn register_all_components(w: &mut specs::World) {
    w.register::<Hostile>();
    w.register::<Position>();
}

// Marker/Stateless Components
#[derive(Debug, PartialEq, Eq, Hash, Component)]
pub struct Hostile {}

// Stateful Components
#[derive(Debug, PartialEq, Eq, Hash, Component)]
pub struct Position(Coords);

