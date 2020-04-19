
use {
    std::{
        fmt::{self, Formatter, Display},
    },
    crate::{
        cpu::Cpu,
    },
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ArgType {
    Address,
    Value,
    Unknown,
    Either,
    String,
    Char,
    Register,
} impl ArgType {
    pub fn try_from(v: u8) -> Option<Self> {
        match v & 0x70 {
            0x00 => Some(Self::Unknown),
            0x10 => Some(Self::Address),
            0x20 => Some(Self::Value),
            0x30 => Some(Self::Either),
            0x40 => Some(Self::String),
            0x50 => Some(Self::Char),
            0x60 => Some(Self::Register),
            _ => None
        }
    }
} impl Display for ArgType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Address => "addr",
            Self::Value => "val",
            Self::Unknown => "unknwn",
            Self::Either => "ethr",
            Self::String => "str",
            Self::Char => "char",
            Self::Register => "reg",
        })
    }
} impl Into<u8> for ArgType {
    fn into(self) -> u8 {
        match self {
            Self::Unknown => 0x00,
            Self::Address => 0x10,
            Self::Value => 0x20,
            Self::Either => 0x30,
            Self::String => 0x40,
            Self::Char => 0x50,
            Self::Register => 0x60,
        }
    }
}

pub struct InstructionFlags {
    nargs: u8,
    is_internal: bool,
} impl InstructionFlags {
    pub fn new() -> InstructionFlags {
        InstructionFlags {
            nargs: 0,
            is_internal: false,
        }
    }

    pub fn is_internal(mut self, is_internal: bool) -> Self {
        self.is_internal = is_internal;
        self
    }

    pub fn nargs(mut self, nargs: u8) -> Self {
        self.nargs = nargs;
        self
    }

    pub fn build(self) -> u8 {
        self.into()
    }

} impl From<u8> for InstructionFlags {
    fn from(v: u8) -> Self {
        Self {
            nargs: v & 0x0F,
            is_internal: v & 0x10 != 0,
        }
    }
} impl Into<u8> for InstructionFlags {
    fn into(self) -> u8 {
        let mask = if self.is_internal {
            self.nargs | 0x10
        } else {
            self.nargs
        };
        eprintln!("{:b} -> {:b}", mask, mask | 0x80);
        mask | 0x40
    }
}

pub struct ArgFlags {
    arg_idx: u8,
    arg_type: ArgType,
} impl ArgFlags {
    pub fn new() -> Self {
        Self {
            arg_idx: 0,
            arg_type: ArgType::Unknown,
        }
    }

    pub fn arg_idx(mut self, arg_idx: u8) -> Self {
        self.arg_idx = arg_idx;
        self
    }

    pub fn arg_type(mut self, arg_type: ArgType) -> Self {
        self.arg_type = arg_type;
        self
    }

    pub fn build(self) -> u8 {
        self.into()
    }
} impl From<u8> for ArgFlags {
    fn from(v: u8) -> Self {
        Self {
            arg_idx: v & 0x0F,
            arg_type: ArgType::try_from(v).unwrap_or(ArgType::Unknown),
        }
    }
} impl Into<u8> for ArgFlags {
    fn into(self) -> u8 {
        let a: u8 = 0x80 & self.arg_idx;
        let b: u8 = self.arg_type.into();
        a & b
    }
}

pub fn strip_prefixes(s: String) -> String {
    let mut s = s;
    "\"'#$%".chars().for_each(|filter| s = s.replace(filter, ""));
    s
}

pub fn parse_byte(s: String) -> u8 {
    if s.starts_with("0x") {

        match hex::decode(s.get(2..).unwrap_or("")) {
            Ok(v) => *v.first().unwrap_or(&0),
            Err(e) => {
                eprintln!("e: {}", e);
                0
            }
        }
    } else {
        match s.parse::<u8>() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Parse Error: {:?}", e);
                0
            }
        }
    }
}

pub fn get_arg_type_and_byte(arg_str: String, expected: ArgType, cpu: &Cpu) -> (ArgType, u8) {
    let has_specifier = "\''#$%".chars().any(|filter| arg_str.starts_with(filter));
    let calls_register = arg_str.contains("%");

    let mut arg_type = match arg_str.get(0..=0).unwrap() {
        "\"" => ArgType::String,
        "'" => ArgType::Char,
        "#" => ArgType::Address,
        "$" => ArgType::Value,
        "%" => ArgType::Register,
        _ => ArgType::Unknown
    };

    let arg_str = if calls_register {
        if arg_type == ArgType::Register { arg_type = expected };
        let reg_key = strip_prefixes(arg_str);
        if reg_key == "a" {
            format!("0x{:X}", cpu.reg_a)
        } else {
            format!("0x{:X}", cpu.reg_b)
        }
    } else if arg_type == ArgType::String {
        // TODO this
        return (arg_type, string)
    } else {
        strip_prefixes(arg_str)
    };

    (arg_type, parse_byte(arg_str))
}