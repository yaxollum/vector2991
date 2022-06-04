use std::fmt;

use super::vector::Vector;

#[derive(Debug, PartialEq)]
pub enum Ast {
    ReprQuad(Vector),
    ReprTrue(Vector),
    ReprCoord(Vector),
    NOP,
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReprCoord(v) => writeln!(f, "({:.2},{:.2})", v.x, v.y),
            Self::ReprTrue(v) => {
                let ang = v.abs_bearing();
                let mag = v.magnitude();
                writeln!(f, "{:.2} [{:.2}]", mag, ang)
            }
            Self::ReprQuad(v) => {
                let ang = v.abs_bearing();
                let mag = v.magnitude();
                writeln!(f, "{:.2} [{}]", mag, Quad(ang.round() as i64))
            }
            Self::NOP => Ok(()),
        }
    }
}

struct Quad(i64);

impl fmt::Display for Quad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ang = self.0;
        if ang == 0 {
            write!(f, "N")
        } else if ang < 90 {
            write!(f, "N{}E", ang)
        } else if ang == 90 {
            write!(f, "E")
        } else if ang < 180 {
            write!(f, "S{}W", 180 - ang)
        } else if ang == 180 {
            write!(f, "S")
        } else if ang < 270 {
            write!(f, "S{}E", ang - 180)
        } else if ang == 270 {
            write!(f, "W")
        } else {
            write!(f, "N{}W", 360 - ang)
        }
    }
}
