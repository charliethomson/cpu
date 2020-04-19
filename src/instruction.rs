
use {
    crate::{
        cell::Cell,
        util::{
            InstructionFlags,
            ArgFlags,
            ArgType,
            parse_byte,
            strip_prefixes,
            get_arg_type_and_byte
        },
        cpu::Cpu,
    },
};

pub fn byte_from_str(s: String) -> u8 {
    match s.as_str() {
        "nop" => 0x00,
        "sta" => 0x01,
        "stb" => 0x02,
        "lda" => 0x03,
        "ldb" => 0x04,
        "lt" => 0x10,
        "le" => 0x11,
        "eq" => 0x12,
        "ge" => 0x13,
        "gt" => 0x14,
        "jmp" => 0x20,
        "ji" => 0x21,
        "in" => 0x30,
        "out" => 0x31,
        "push" => 0x32,
        "pop" => 0x33,
        "adda" => 0x40,
        "addb" => 0x41,
        "suba" => 0x42,
        "subb" => 0x43,
        _ => 0x00,
    }
}

pub fn targs(s: String) -> Vec<ArgType> {
    match s.as_str() {
        "nop" => vec![],
        "sta" => vec![ArgType::Address],
        "stb" => vec![ArgType::Address],
        "lda" => vec![ArgType::Address],
        "ldb" => vec![ArgType::Address],
        "lt" => vec![],
        "le" => vec![],
        "eq" => vec![],
        "ge" => vec![],
        "gt" => vec![],
        "jmp" => vec![ArgType::Address],
        "ji" => vec![ArgType::Address],
        "in" => vec![ArgType::Address],
        "out" => vec![ArgType::Address, ArgType::Value],
        "push" => vec![ArgType::Value, ArgType::Either],
        "pop" => vec![ArgType::Address],
        "adda" => vec![ArgType::Either],
        "addb" => vec![ArgType::Either],
        "suba" => vec![ArgType::Either],
        "subb" => vec![ArgType::Either],
        _ => vec![],      
    }
}

pub fn nargs(s: String) -> u8 {
    targs(s).len() as u8
}
// TODO: Implement Display
#[derive(Debug)]
pub struct Instruction {
    pub instruction: Cell,
    pub args: Vec<Cell>,
} impl Instruction {
    pub fn from_str<S: Into<String>>(s: S, cpu: &Cpu) -> Result<Result<Self, String>, String> {
        let s = s.into();

        if s.trim().starts_with("'") {
            return Ok(Err("Comment".to_owned()));
        }

        let parts = s.split_whitespace().map(|s| s.trim().to_owned()).collect::<Vec<String>>();

        if parts.len() == 0 {
            return Ok(Err("Passed an empty string".to_owned()));
        }
        let nargs = parts.len() - 1;

        let mut parts_iter = parts.iter();

        // safe because of the check above
        let instr_str = parts_iter.next().unwrap();

        let instruction = Cell::from({
            
            let high = InstructionFlags::new()
                        .is_internal(false)
                        .nargs(nargs as u8)
                        .build();

            let low = byte_from_str(instr_str.clone());
            ((high as u16) << 8u16) | low as u16
        });

        let mut targs = targs(instr_str.clone());
        eprintln!("{:?}", targs);
        while targs.len() < parts.len() - 1 {
            targs.push(ArgType::Unknown)
        }
        eprintln!("{:?}", targs);
        let contains_str_arg = parts_iter.clone().any(|part| part.contains("\""));

        let mut parts = vec![];

        // TODO: Fix this please
        if contains_str_arg {
            let args_str = parts_iter.clone().map(|s| s.to_owned()).collect::<String>();
            let mut part = String::new();
            let mut in_str = false;
            for c in args_str.chars() {
                match c {
                    '"' => {
                        if in_str {
                            in_str = false;
                            parts.push(part.clone());
                            part = String::new();
                        } else {
                            in_str = true;
                            part.push(c);
                        }
                    },
                    _ => {
                        if c.is_whitespace() {
                            if in_str {
                                part.push(c);
                            } else {
                                parts.push(part.clone());
                                part = String::new();
                            }
                        }
                    }
                }
            }

            parts_iter = parts.iter();
        }

        let args = parts_iter.zip(targs.iter()).enumerate().fold(Vec::new(), |mut acc, (idx, (arg_str, arg_type))| {
            if arg_str.is_empty() {
                return acc;
            }
            let (arg_type, low) = get_arg_type_and_byte(arg_str.clone(), *arg_type, cpu);
            if arg_type == ArgType::String {

                acc
            } else {
                let high = ArgFlags::new()
                .arg_idx(idx as u8)
                .arg_type(arg_type)
                .build();
                
                
                eprintln!("High: {} -> ({:X} {:b})", high, high, high);
                eprintln!("Low: {} -> ({:X} {:b})", low, low, low);
                acc.push(Cell::from(((high as u16) << 8u16) | low as u16));
                acc
            }
        });

        eprintln!("Args: {:?}", args);

        Ok(Ok(Self {
            instruction,
            args
        }))
    }

    pub fn sz(&self) -> usize {
        self.args.len() + 1
    }
}