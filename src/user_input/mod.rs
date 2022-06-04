//Jerome M. St.Martin
//May, 2022


use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::error::Gremlin;
use crate::common::{Dir, Message};

#[derive(Debug)]
pub struct UserInput {}

impl UserInput {
    pub(crate) fn blocking_read() -> Result<Message, Gremlin> {

        let event = crossterm::event::read()?;
        
        //println!("{:?}\r", event); //For Testing Only

        Ok(Self::translate(event))
    }

    fn translate(event: Event) -> Message {
        let mut msg = Message::Null;

        match event {
            Event::Key(key_event) => {
                let code = key_event.code;
                let mods = key_event.modifiers; //No need at this time
                match code {
                    KeyCode::Backspace => { msg = Message::Delete },
                    KeyCode::Enter => { msg = Message::Confirm },
                    KeyCode::Left => { msg = Message::HJKL(Dir::W) },
                    KeyCode::Right => { msg = Message::HJKL(Dir::E) },
                    KeyCode::Up => { msg = Message::HJKL(Dir::N) },
                    KeyCode::Down => { msg = Message::HJKL(Dir::S) },
                    KeyCode::Home => {},
                    KeyCode::End => {},
                    KeyCode::PageUp => {},
                    KeyCode::PageDown => {},
                    KeyCode::Tab => { msg = Message::Tab },
                    KeyCode::BackTab => { msg = Message::BackTab },
                    KeyCode::Delete => { msg = Message::Delete },
                    KeyCode::Insert => {},
                    KeyCode::F(_) => { msg = Message::Menu },
                    KeyCode::Char(c) => {
                        match c {
                            //WASD
                            'w' => { msg = Message::WASD(Dir::N) },
                            'e' => { msg = Message::WASD(Dir::NE) },
                            'd' => { msg = Message::WASD(Dir::E) },
                            'c' => { msg = Message::WASD(Dir::SE) },
                            's' => { msg = Message::WASD(Dir::S) },
                            'z' => { msg = Message::WASD(Dir::SW) },
                            'a' => { msg = Message::WASD(Dir::W) },
                            'q' => { msg = Message::WASD(Dir::NW) },
                            
                            //HJKL
                            'h' => { msg = Message::HJKL(Dir::W) },
                            'j' => { msg = Message::HJKL(Dir::S) },
                            'k' => { msg = Message::HJKL(Dir::N) },
                            'l' => { msg = Message::HJKL(Dir::E) },

                            _ => {},
                        }
                    },
                    KeyCode::Null => {},
                    KeyCode::Esc => { msg = Message::Cancel; },
                }

                match mods {
                    KeyModifiers::SHIFT => {},
                    KeyModifiers::CONTROL => {
                        if code == KeyCode::Char('c') { msg = Message::Exit; }
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
