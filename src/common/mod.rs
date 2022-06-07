//Jerome M. St.Martin
//May, 2022

use specs::Entity;

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
    Map(Box<MapMutation>),
    Exit,
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



///Used as the inner Ok() type for the various .tick() methods' returned Results.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Ticker {
    ExitProgram,
    Continue,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Coords { pub x: u16, pub y: u16 }

impl Coords {
    pub fn new<T: Into<u16>>(x: T, y: T) -> Self {
        Coords {x: x.into(), y: y.into()}
    }
}
