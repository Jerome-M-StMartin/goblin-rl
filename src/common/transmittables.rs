//Jerome M. St.Martin
//June 10, 2022

//-----------------------------------------------------------------------------
//--------------------- Things that Get Sent via Channels ---------------------
//-----------------------------------------------------------------------------

use super::Dir;

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
    Exit,
}
//------------------------ ------------- ------------------------

//------------------------ Model -> View ------------------------
///Commands passed from Model to View (in MVC) via mpsc::channels.
///i.e. The Model telling the View: "Here's what changed in the Game World".
#[derive(PartialEq, Eq, Debug)]
pub enum DeltaNotification {
    MapDirty,
}

//------------------------ ------------- ------------------------
