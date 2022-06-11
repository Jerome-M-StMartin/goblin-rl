//Jerome M. St.Martin
//June 10, 2022

//-----------------------------------------------------------------------------
//--------------------- Things that Get Sent via Channels ---------------------
//-----------------------------------------------------------------------------

use specs::Entity;

use crate::ecs_access_point::AccessKey;

use super::Dir;

use usize as Index; //Type Alias

//---------------------- Controller -> View ----------------------
///Commands passed from Controller to View (in MVC) via mpsc::channels.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum InputEvent {
    HJKL(Dir),
    WASD(Dir),
    Cancel,
    Confirm,
    Tab,
    BackTab,
    Delete,
    Menu,
    Null,
    Exit, //Used to end the program
}
//------------------------ ------------- -------------------------

//---------------------- View -> Model ---------------------
///Commands passed from Controller to Model (in MVC) via mpsc::channels.
///i.e. The Controller telling the Model: "Change the Game World in this way".
#[derive(PartialEq, Eq, Debug)]
pub enum MutateCommand {
    Test,
    InsertMarkerComponent(Box<InsertionData>),
    Map(Box<MapMutation>),
    Exit,
}

#[derive(PartialEq, Eq, Debug)]
pub struct InsertionData {
    key: AccessKey,
    target: Entity,
}

///The inner value for MutateCommand variant "Map".
#[derive(PartialEq, Eq, Debug)]
pub struct MapMutation {
    idx: Index,
    add: Option<Entity>,
    rm: Option<Entity>,
}
//------------------------ ------------- ------------------------

//------------------------ Model -> View ------------------------
///Commands passed from Model to View (in MVC) via mpsc::channels.
///i.e. The Model telling the View: "Here's what changed in the Game World".
#[derive(PartialEq, Eq, Debug)]
pub enum DeltaNotification {
    Map(Box<MapDelta>),
}

///The inner value for DeltaNotification variant "Map".
#[derive(PartialEq, Eq, Debug)]
pub struct MapDelta {
    idx: Vec<Index>,
    current_content: Vec<[Entity; 8]>,
}
//------------------------ ------------- ------------------------
