//Jerome M. St.Martin
//May, 2022

use crossterm::event::{poll, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

use super::super::error::Gremlin;

//-------------------------------------------
//--------------- CONTROLLER ----------------
//----------------- of MVC ------------------
//-------------------------------------------

pub struct UserInput {
    event: Option<Event>,
}

impl UserInput {
    pub fn new() -> Self {
        UserInput { event: None }
    }

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
        } else { /* poll timeout expired */
        };

        Ok(Event::Key(KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)))
    }

    pub fn tick(&mut self) -> Result<(), Gremlin> {
        let event = Self::read()?;
        self.store(event);

        //TODO

        Ok(())
    }

    fn store(&mut self, e: Event) {
        self.event = Some(e);
    }
}
