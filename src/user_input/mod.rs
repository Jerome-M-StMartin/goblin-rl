//Jerome M. St.Martin
//May, 2022

use std::time::Duration;

use crossterm::event::{poll, Event, KeyEvent, KeyCode, KeyModifiers};

use super::error::Gremlin;

//-------------------------------------------
//--------------- CONTROLLER ----------------
//----------------- of MVC ------------------
//-------------------------------------------

#[derive(Debug)]
pub struct UserInput {
    event: Option<Event>,
}

impl UserInput {
    pub fn new() -> Self {
        UserInput {
            event: None,
        }
    }

    //Does Not Block (assuming it's correctly written)
    fn read_crossterm() -> std::io::Result<Event> {

        if poll(Duration::from_millis(500))? { //TODO Confirm this duration is desireable. Try 0.
            match crossterm::event::read()? {
                event => return Ok(event),
            }
        }
        
        //poll timeout expired - so return a "non-event event"
        Ok(Event::Key(KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)))
    }

    pub fn tick(&mut self) -> Result<(), Gremlin> {

        let event = Self::read_crossterm()?;
        let non_event = Event::Key(KeyEvent::new(KeyCode::Null, KeyModifiers::NONE));

        if event == non_event {
            self.event = None;
        } else {
            self.event = Some(event);
        }
        
        Ok(())
    }

    //This should never be called BEFORE self.tick() in any given frame.
    pub fn take_input_event(&mut self) -> Option<Event> {
        //Removes self.event's Event from its option,
        //leaving None in its place, even if it was None to begin with,
        //in which case take() also returns None.
        self.event.take() 
    }
}




    /*fn blocking_read() -> crossterm::Result<Event> {
        /*match read()? {
            event => return Ok(event),
        }*/
        crossterm::event::read()
    }*/
