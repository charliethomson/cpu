
use {
    std::{
        io::{ Read },
        path::{ Path },
        fs::{ File },

    },
    crate::{
        cell::Cell,
        instruction::Instruction,
        SUPPRESS_WARNINGS,
    }
};

pub const MEMORY_NBYTES: usize = 128;

pub struct Cpu {
    pub memory: [Cell; MEMORY_NBYTES],
    pub reg_a: u8,
    pub reg_b: u8,
    pub ip: usize,
    pub sbp: usize,
    pub sp: usize,
} impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: [Cell::zero(); MEMORY_NBYTES],
            reg_a: 0,
            reg_b: 0,
            ip: 0,
            sbp: 0,
            sp: 0,
        }
    }

    pub fn with_file<P: AsRef<Path>>(mut self, path: P) -> Result<Self, String> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(format!("Encountered an error opening the file: {}", e))
        };

        let mut buffer = String::new();
        file.read_to_string(&mut buffer);
        let mut dp = 0;
        
        for line in buffer.split_terminator("\n") {
            let instruction = match Instruction::from_str(line, &self) {
                Ok(res_warn) => {
                    match res_warn {
                        Ok(instruction) => Some(instruction),
                        Err(warning) => {
                            if !SUPPRESS_WARNINGS {
                                eprintln!("{}", warning);
                            }
                            None
                        }
                    }
                },
                Err(e) => return Err(e)
            };
            if let Some(instruction) = instruction {
                let nbytes = instruction.sz();
                if dp + nbytes >= MEMORY_NBYTES {
                    return Err(format!("Memory overflow when reading in {:?}", instruction));
                }

                self.memory[dp] = instruction.instruction;
                unsafe {
                    // This is safe because of the check above
                    std::ptr::copy(
                        instruction.args.as_ptr(), 
                        self.memory.as_mut_ptr().add(dp + 1),
                        nbytes - 1
                    );
                }
                eprintln!("Instruction: {:?}", instruction);

                dp += nbytes;
            }
        }

        eprintln!("{:?}", self.memory.to_vec());

        Ok(self)
    }

    pub fn step(&mut self) -> Result<Result<(), String>, String> {
        
        Ok(Ok(()))
    }

    pub fn run(mut self) -> Result<(), String> {
        loop {
            match self.step() {
                Ok(res_warn) => {
                    match res_warn {
                        Ok(_) => (),
                        Err(e) => {

                        }
                    }
                },
                Err(e) => return Err(e)
            }
        }
        Ok(())
    }
}