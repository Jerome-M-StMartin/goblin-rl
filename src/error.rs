//Jerome M. St.Martin
//May, 2022

use std::fmt;

use crate::common::Message;

//-------------------------------------------
//------------ Custom Error Type ------------
//-------------- & Error Codes --------------
//-------------------------------------------

#[derive(Debug)]
pub enum Gremlin { //add variants as needed
    //Internal Errors
    InvalidInput,
    
    //Outside Errors w/ Source Fields
    IOError(std::io::Error),
    SendError(std::sync::mpsc::SendError<Message>),
    TryRecvErr(std::sync::mpsc::TryRecvError),
}

impl fmt::Display for Gremlin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> std::error::Error for Gremlin {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Gremlin::IOError(source) => Some(source),
            Gremlin::SendError(source) => Some(source),
            Gremlin::TryRecvErr(source) => Some(source),
            _ => { None },
        }
    }
}

impl<'a> From<std::io::Error> for Gremlin {
    fn from(item: std::io::Error) -> Self {
        Gremlin::IOError(item)
    }
}

impl<'a> From<std::sync::mpsc::SendError<Message>> for Gremlin {
    fn from(item: std::sync::mpsc::SendError<Message>) -> Self {
        Gremlin::SendError(item)
    }
}

impl<'a> From<std::sync::mpsc::TryRecvError> for Gremlin {
    fn from(item: std::sync::mpsc::TryRecvError) -> Self {
        Gremlin::TryRecvErr(item)
    }
}
