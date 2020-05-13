use crate::{
    asm::error::AssemblerError, cpu::Cpu, error::CpuError, instruction::Instruction, DEBUG,
};
use std::{
    collections::HashMap,
    convert::TryFrom,
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub struct Assembler {
    labels: HashMap<String, usize>,
    input: File,
    output: Vec<u8>,
    unknown_labels: HashMap<String, Vec<usize>>,
}
impl Assembler {
    pub fn new(file_handle: File) -> Self {
        Self {
            labels: HashMap::new(),
            input: file_handle,
            output: vec![],
            unknown_labels: HashMap::new(),
        }
    }

    pub fn jit(file_handle: File, cpu: &mut Cpu) -> Result<(), CpuError> {
        Ok(())
    }

    pub fn parse(&mut self) -> Result<usize, AssemblerError> {
        let mut lines = String::new();
        match self.input.read_to_string(&mut lines) {
            Ok(_nbytes) => (),
            Err(e) => return Err(AssemblerError::IOError(e)),
        };
        let lines = lines.split_terminator('\n');
        for (line_no, line) in lines
            .map(|line| line.split_terminator(";;").next().unwrap_or(""))
            .enumerate()
        {
            if line.chars().all(|c| c.is_whitespace()) || line.is_empty() {
                continue;
            }

            if DEBUG {
                eprintln!("Parsing {:?}", line);
            }
            if !" \t".chars().any(|filter| line.starts_with(filter)) {
                // This _should_ be a label
                let line = line.trim();
                if !line.ends_with(':') {
                    return Err(AssemblerError::UnexpectedInstruction(
                        line.to_owned(),
                        line_no,
                    ));
                } else {
                    if DEBUG {
                        eprintln!(
                            "Adding label {} -> {}",
                            line[..line.len() - 1].to_owned(),
                            self.output.len()
                        );
                    }
                    self.labels.insert(
                        line[..line.len() - 1].to_owned(),
                        self.output.len().checked_sub(1).unwrap_or(0),
                    );
                }
            } else {
                // Instruction
                let line = line.trim();
                match Instruction::try_from(line.to_owned()) {
                    Ok(instruction) => {
                        let bytes = instruction.as_bytes();
                        if DEBUG {
                            eprintln!("For {}, pushing {:?}", line, bytes);
                        }
                        self.output.extend(bytes);
                    }
                    Err(e) => match e {
                        Ok((instruction_byte, arg_str)) => {
                            if let Some(arg_byte) = self.labels.get(&arg_str) {
                                if DEBUG {
                                    eprintln!(
                                        "For {}, pushing [{}, {}]",
                                        line, instruction_byte, arg_byte
                                    );
                                }

                                if let Some(addrs) = self.unknown_labels.remove(&arg_str) {
                                    for addr in addrs {
                                        self.output.remove(addr);
                                        self.output.insert(addr, *arg_byte as u8)
                                    }
                                }
                                self.output.push(instruction_byte);
                                self.output.push(*arg_byte as u8);
                            } else {
                                self.output.push(instruction_byte);
                                if self.unknown_labels.contains_key(&arg_str) {
                                    self.unknown_labels
                                        .get_mut(&arg_str)
                                        .unwrap()
                                        .push(self.output.len());
                                } else {
                                    self.unknown_labels.insert(arg_str, vec![self.output.len()]);
                                }
                                self.output.push(0);
                            }
                        }
                        Err(e) => return Err(AssemblerError::InstructionError(e)),
                    },
                }
            }
        }

        for (label, addrs) in self.unknown_labels.iter() {
            if !self.labels.contains_key(label) {
                return Err(AssemblerError::UndefinedLabel(label.to_owned()));
            } else {
                let label_addr = *(self.labels.get(label).unwrap());
                for addr in addrs {
                    if label_addr > 255 {
                        panic!("How the fuck did you get a label outside of memory... dummy")
                    }
                    if *addr > self.output.len() {
                        panic!("How did this even happen. You got an instruction that points to a label to go outta bounds. good for you. Dick.")
                    }
                    self.output.remove(*addr);
                    self.output.insert(*addr, label_addr as u8);
                }
            }
        }

        Ok(self.output.len())
    }

    pub fn run(&self) -> Result<(), CpuError> {
        let mut cpu = Cpu::new();
        cpu.memory = crate::memory::Memory::new_with_instructions(&self.output);
        cpu.run()
    }

    pub fn load_and_run(&self, cpu_handle: &mut Cpu) -> Result<(), CpuError> {
        cpu_handle.memory = crate::memory::Memory::new_with_instructions(&self.output);
        cpu_handle.run()
    }

    pub fn output_to_file<P: AsRef<Path>>(self, path: P) -> std::io::Result<()> {
        let mut file_handle = File::open(path)?;
        file_handle.write(&self.output)?;
        Ok(())
    }

    pub fn get_output(self) -> Vec<u8> {
        self.output
    }
}
