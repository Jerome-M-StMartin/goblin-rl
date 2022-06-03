//Jerome M. St.Martin
//May, 2022

use std::fmt;

//-------------------------------------------
//------------ Custom Error Type ------------
//-------------- & Error Codes --------------
//-------------------------------------------

pub struct Gremlin {
    code: GCODE,
    source: Option<std::io::Error>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GCODE {
    //Gremlin Identity Code
    GW,
    GUI,
    Threading,
    InvalidInput,
    OutsideErr,
    Undefined,
}

impl Gremlin {
    pub fn new(code: GCODE, source: Option<std::io::Error>) -> Self {
        Gremlin {
            code,
            source,
        }
    }
}

fn translate_code(code: GCODE) -> &'static str {
    match code {
        GCODE::GW => "eneral GameWorld error",
        GCODE::GUI => "general GUI error",
        GCODE::Threading => "multithreading error",
        GCODE::InvalidInput => "fn input is invalid",

        GCODE::OutsideErr => "outside error",
        GCODE::Undefined => "undefined error",
    }
}

impl fmt::Display for GCODE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", translate_code(*self))
    }
}

impl<'a> fmt::Display for Gremlin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = translate_code(self.code);
        write!(f, "{}", err_msg)
    }
}

impl<'a> fmt::Debug for Gremlin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.code, translate_code(self.code))
    }
}

impl<'a> std::error::Error for Gremlin {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Some(source) = &self.source {
            return Some(source);
        }
        None
    }
}

impl<'a> From<std::io::Error> for Gremlin {
    fn from(item: std::io::Error) -> Self {
        Gremlin {
            code: GCODE::OutsideErr,
            source: Some(item),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write as FmtWrite;

    #[test]
    fn test_0() {
        let e = Gremlin {
            code: GCODE::GW,
            source: None,
        };
        assert!(format!("{}", e) == translate_code(GCODE::GW));
    }

    #[test]
    fn test_1() {
        let e = Gremlin {
            code: GCODE::GUI,
            source: None,
        };
        assert!(format!("{}", e) == translate_code(GCODE::GUI));
    }

    #[test]
    fn test_2() {
        let e = Gremlin {
            code: GCODE::Threading,
            source: None,
        };
        let mut buff = String::new();
        write!(&mut buff, "{}: {}", e.code, translate_code(e.code)).unwrap();
        assert!(format!("{:?}", e) == buff);
    }
}
