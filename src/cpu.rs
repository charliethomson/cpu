use {
    crate::{
        error::CpuError, flags::Flags, instruction::Instruction, memory::Memory, DEBUG, MEMORY_SIZE,
    },
    std::{fs::File, io::Read, path::Path},
};

pub struct Cpu {
    pub ip: u8,
    pub memory: Memory,
    pub flags: Flags,
    pub user: u8,
    pub accumulator: u8,
}
impl Cpu {
    pub fn new() -> Self {
        Self {
            ip: 0,
            memory: Memory::new(),
            flags: Flags::default(),
            user: 0,
            accumulator: 0,
        }
    }

    // pub fn from_file<P: AsRef<Path>>(path: P) -> Self {}

    pub fn from_binary<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut handle = File::open(path)?;
        let mut bytes = Vec::new();
        handle.read_to_end(&mut bytes)?;

        let memory = Memory::new_with_instructions(&bytes);
        let flags = Flags::default();

        Ok(Cpu {
            ip: 0,
            memory,
            flags,
            user: 0,
            accumulator: 0,
        })
    }

    pub fn run(&mut self) -> Result<(), CpuError> {
        loop {
            self.step()?;
        }
    }

    pub fn step(&mut self) -> Result<(), CpuError> {
        if self.ip >= 0xFF || self.ip as usize >= MEMORY_SIZE {
            return Err(CpuError::AOverflow);
        }

        let instruction = self.memory.get(self.ip)?;
        let instruction = match Instruction::from_byte(instruction) {
            Some(single_byte) => single_byte,
            None => {
                self.ip += 1;
                let arg = self.memory.get(self.ip)?;
                Instruction::from_byte_and_arg(instruction, arg)?
            }
        };

        if DEBUG {
            eprintln!("({}) -> {:?}", String::from(instruction), self);
        }

        instruction.execute(self)?;

        self.ip += 1;

        if DEBUG {
            eprintln!("After: {:?}", self);
        }

        if self.flags.clear {
            self.flags.clear_flags();
        } else if self.flags.any() {
            self.flags.clear = true;
        } else {
            if self.accumulator == 0 || self.user == 0 {
                self.flags.zero = true;
            }
        }

        Ok(())
    }
}
impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cpu {{\n\tip: {};\n\tflags: {:?};\n\tregisters: {{\n\t\tusr: {};\n\t\tacc: {}\n\t}}\n\tmemory: {:?}\n}};",
            self.ip, self.flags, self.user, self.accumulator, self.memory,
        )
    }
}
