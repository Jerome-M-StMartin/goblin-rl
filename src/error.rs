//Jerome M. St.Martin
//May, 2022

use std::fmt;

//-------------------------------------------
//------------ Custom Error Type ------------
//-------------- & Error Codes --------------
//-------------------------------------------

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Gremlin {
    code: GCODE,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GCODE {//Gremlin Identity Code
    GW,
    GUI,
    Threading,
    Undefined,
}

fn translate_code(code: GCODE) -> &'static str {
    match code {
        GCODE::GW => "eneral GameWorld error",
        GCODE::GUI => "general GUI error",
        GCODE::Threading => "multithreading error",
        GCODE::Undefined => "undefined error",
    }
}

impl fmt::Display for GCODE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", translate_code(*self))
    }
}

impl fmt::Display for Gremlin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = translate_code(self.code);
        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for Gremlin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}: {}",
               self.code,
               translate_code(self.code)
        )
    }
}

impl std::error::Error for Gremlin {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write as FmtWrite;

    #[test]
    fn test_0() {
        let e = Gremlin { code: GCODE::GW };
        assert!(format!("{}", e) == translate_code(GCODE::GW));
    }

    #[test]
    fn test_1() {
        let e = Gremlin { code: GCODE::GUI };
        assert!(format!("{}", e) == translate_code(GCODE::GUI));
    }

    #[test]
    fn test_2() {
        let e = Gremlin { code: GCODE::Threading };
        let mut buff = String::new();
        write!(&mut buff, "{}: {}", e.code, translate_code(e.code)).unwrap();
        assert!(format!("{:?}", e) == buff);
    }
}
