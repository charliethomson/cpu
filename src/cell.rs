
use {
    std::{
        fmt::{self, Formatter, Display, Debug},
    },
    crate::{
        util::ArgType,
    }
};



#[derive(Clone, Copy)]
pub struct Cell {
    pub high: u8,
    pub low: u8,
} impl Cell {
    pub fn zero() -> Self {
        Self::from(0)
    }

    pub fn is_instruction(&self) -> bool {
        self.high & 0x40 != 0
    }

    pub fn is_argument(&self) -> bool {
        self.high & 0x80 != 0
    }

    pub fn arg_type(&self) -> Option<ArgType> {
        if !self.is_argument() {
            None
        } else {
            ArgType::try_from(self.high)
        }
    }
    
    pub fn arg_idx(&self) -> Option<u8> {
        if !self.is_argument() {
            None
        } else {
            Some(self.high & 0x0F)
        }
    }
} impl From<u16> for Cell {
    fn from(v: u16) -> Self {
        Self {
            high: ((v & 0xFF00) >> 8) as u8,
            low: (v & 0x00FF) as u8
        }
    }
} impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}", self.low)
    }
} impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{:X}", 
            if self.is_argument() {
                format!("arg{}({})", self.arg_idx().unwrap(), self.arg_type().unwrap())
            } else if self.is_instruction() {
                if (self.high & 0x10) != 0 {
                    "intern".to_owned()
                } else {
                    "instr".to_owned()
                }
            } else {
                "Unknown".to_owned()
            }, self.low
        )
    }
}