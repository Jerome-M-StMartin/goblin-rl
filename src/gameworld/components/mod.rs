//Jerome M. St.Martin
//June 6, 2022

//-----------------------------------------------------------------------------
//--------------------------- ECS Component Module ----------------------------
//-----------------------------------------------------------------------------

use specs::prelude::*;
use specs_derive::Component;

pub(crate) fn register_all_components(w: &mut specs::World) {
    w.register::<TestComponent>();
}

#[derive(Debug, PartialEq, Eq, Hash, Component)]
pub struct TestComponent {}
