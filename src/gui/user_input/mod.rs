//Jerome M. St.Martin
//May, 2022

use std::time::Duration;
use crossterm::event::{poll, Event, KeyEvent, KeyCode, KeyModifiers};

//-------------------------------------------
//--------------- CONTROLLER ----------------
//----------------- of MVC ------------------
//-------------------------------------------

pub struct UserInput {
    event: Option<Event>,
}

impl UserInput {
    pub fn new() -> Self {
        UserInput {
            event: None,
        }
    }

    pub fn tick(&mut self) {}

    /*fn blocking_read() -> crossterm::Result<Event> {
        /*match read()? {
            event => return Ok(event),
        }*/
        crossterm::event::read()
    }*/

    //If written correctly: Does Not Block
    fn read() -> crossterm::Result<Event> {

        if poll(Duration::from_millis(500))? {

            match crossterm::event::read()? {
                event => return Ok(event),
            }

        } else { /* poll timeout expired */ };

        Ok(Event::Key(KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)))
    }
}
