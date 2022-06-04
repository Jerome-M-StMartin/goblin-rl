//Jerome M. St.Martin
//May, 2022

use std::fmt;

use crate::common::Message;

//-------------------------------------------
//------------ Custom Err Type ------------
//-------------- & Err Codes --------------
//-------------------------------------------

#[derive(Debug)]
pub enum Gremlin { //add variants as needed
    //Internal Errs
    InvalidInput,
    
    //Outside Errs w/ Source Fields
    IOErr(std::io::Error),
    SendErr(std::sync::mpsc::SendError<Message>),
    RecvErr(std::sync::mpsc::RecvError),
}

impl fmt::Display for Gremlin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> std::error::Error for Gremlin {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Gremlin::IOErr(source) => Some(source),
            Gremlin::SendErr(source) => Some(source),
            Gremlin::RecvErr(source) => Some(source),
            _ => { None },
        }
    }
}

impl<'a> From<std::io::Error> for Gremlin {
    fn from(item: std::io::Error) -> Self {
        Gremlin::IOErr(item)
    }
}

impl<'a> From<std::sync::mpsc::SendError<Message>> for Gremlin {
    fn from(item: std::sync::mpsc::SendError<Message>) -> Self {
        Gremlin::SendErr(item)
    }
}

impl<'a> From<std::sync::mpsc::RecvError> for Gremlin {
    fn from(item: std::sync::mpsc::RecvError) -> Self {
        Gremlin::RecvErr(item)
    }
}
