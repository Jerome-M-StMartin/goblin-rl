//Jerome M. St.Martin
//May, 2022

use std::fmt;

use super::common::{DeltaNotification, InputEvent, MutateCommand};

//-------------------------------------------
//------------ Custom Err Type ------------
//-------------- & Err Codes --------------
//-------------------------------------------

#[derive(Debug)]
pub enum Gremlin {
    //add variants as needed
    //Internal Errs
    InvalidInput,
    OutOfMapBounds,

    //Outside Errs w/ Source Fields
    IOErr(std::io::Error),
    IESendErr(std::sync::mpsc::SendError<InputEvent>),
    MCSendErr(std::sync::mpsc::SendError<MutateCommand>),
    DNSendErr(std::sync::mpsc::SendError<DeltaNotification>),
    RecvErr(std::sync::mpsc::RecvError),
    SpecsErr(specs::error::Error),
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
            Gremlin::IESendErr(source) => Some(source),
            Gremlin::MCSendErr(source) => Some(source),
            Gremlin::DNSendErr(source) => Some(source),
            Gremlin::RecvErr(source) => Some(source),
            Gremlin::SpecsErr(source) => Some(source),
            _ => None,
        }
    }
}

impl<'a> From<std::io::Error> for Gremlin {
    fn from(item: std::io::Error) -> Self {
        Gremlin::IOErr(item)
    }
}

impl<'a> From<std::sync::mpsc::SendError<InputEvent>> for Gremlin {
    fn from(item: std::sync::mpsc::SendError<InputEvent>) -> Self {
        Gremlin::IESendErr(item)
    }
}

impl<'a> From<std::sync::mpsc::SendError<MutateCommand>> for Gremlin {
    fn from(item: std::sync::mpsc::SendError<MutateCommand>) -> Self {
        Gremlin::MCSendErr(item)
    }
}

impl<'a> From<std::sync::mpsc::SendError<DeltaNotification>> for Gremlin {
    fn from(item: std::sync::mpsc::SendError<DeltaNotification>) -> Self {
        Gremlin::DNSendErr(item)
    }
}

impl<'a> From<std::sync::mpsc::RecvError> for Gremlin {
    fn from(item: std::sync::mpsc::RecvError) -> Self {
        Gremlin::RecvErr(item)
    }
}

impl<'a> From<specs::error::Error> for Gremlin {
    fn from(item: specs::error::Error) -> Self {
        Gremlin::SpecsErr(item)
    }
}
