//Jerome M. St.Martin
//May, 2022


use crossterm::event::{Event, KeyCode, KeyModifiers};

use super::error::Gremlin;
use super::common::{Dir, InputEvent};

#[derive(Debug)]
pub struct UserInput {}

impl UserInput {
    pub(crate) fn blocking_read() -> Result<InputEvent, Gremlin> {

        let event = crossterm::event::read()?;
        
        //println!("{:?}\r", event); //For Testing Only

        Ok(Self::translate(event))
    }

    fn translate(event: Event) -> InputEvent {
        let mut msg = InputEvent::Null;

        match event {
            Event::Key(key_event) => {
                let code = key_event.code;
                let mods = key_event.modifiers; //No need at this time
                match code {
                    KeyCode::Backspace => { msg = InputEvent::Delete },
                    KeyCode::Enter => { msg = InputEvent::Confirm },
                    KeyCode::Left => { msg = InputEvent::Hjkl(Dir::W) },
                    KeyCode::Right => { msg = InputEvent::Hjkl(Dir::E) },
                    KeyCode::Up => { msg = InputEvent::Hjkl(Dir::N) },
                    KeyCode::Down => { msg = InputEvent::Hjkl(Dir::S) },
                    KeyCode::Home => {},
                    KeyCode::End => {},
                    KeyCode::PageUp => {},
                    KeyCode::PageDown => {},
                    KeyCode::Tab => { msg = InputEvent::Tab },
                    KeyCode::BackTab => { msg = InputEvent::BackTab },
                    KeyCode::Delete => { msg = InputEvent::Delete },
                    KeyCode::Insert => {},
                    KeyCode::F(_) => { msg = InputEvent::Menu },
                    KeyCode::Char(c) => {
                        match c {
                            //Wasd
                            'w' => { msg = InputEvent::Wasd(Dir::N) },
                            'e' => { msg = InputEvent::Wasd(Dir::NE) },
                            'd' => { msg = InputEvent::Wasd(Dir::E) },
                            'c' => { msg = InputEvent::Wasd(Dir::SE) },
                            's' => { msg = InputEvent::Wasd(Dir::S) },
                            'z' => { msg = InputEvent::Wasd(Dir::SW) },
                            'a' => { msg = InputEvent::Wasd(Dir::W) },
                            'q' => { msg = InputEvent::Wasd(Dir::NW) },
                            
                            //Hjkl
                            'h' => { msg = InputEvent::Hjkl(Dir::W) },
                            'j' => { msg = InputEvent::Hjkl(Dir::S) },
                            'k' => { msg = InputEvent::Hjkl(Dir::N) },
                            'l' => { msg = InputEvent::Hjkl(Dir::E) },

                            _ => {},
                        }
                    },
                    KeyCode::Null => {},
                    KeyCode::Esc => { msg = InputEvent::Cancel; },
                }

                match mods {
                    KeyModifiers::SHIFT => {},
                    KeyModifiers::CONTROL => {
                        if code == KeyCode::Char('c') { msg = InputEvent::Exit; }
                    },
                    KeyModifiers::ALT => {},
                    KeyModifiers::NONE => {},
                    _ => {},
                }
            },
            Event::Mouse(_mouse_event) => {}, //TODO
            Event::Resize(_x, _y) => {}, //TODO
        };

        msg
    }

}




    /*
    use std::time::Duration;
    //Does Not Block (assuming it's correctly written)
    fn read_crossterm() -> std::io::Result<Event> {

        if crossterm::event::poll(Duration::from_millis(500))? {
            match crossterm::event::read()? {
                event => return Ok(event),
            }
        }
        
        //poll timeout expired - so return a "non-event event"
        Ok(Event::Key(KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)))
    }

        //for tick():
        let non_event = Event::Key(KeyEvent::new(KeyCode::Null, KeyModifiers::NONE));
        if event == non_event {
            self.event = None;
        } else {
            self.event = Some(event);
        }

    //This should never be called BEFORE self.tick() in any given frame.
    pub fn take_input_event(&mut self) -> Option<Event> {
        //Removes self.event's Event from its option,
        //leaving None in its place, even if it was None to begin with,
        //in which case take() also returns None.
        self.event.take() 
    }
    */
