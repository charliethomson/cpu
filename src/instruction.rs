use crate::asm::error::AssemblerError;
use crate::cpu::Cpu;
use crate::error::CpuError;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    LDA(u8),
    STA(u8),
    SETV(u8),
    SETA(u8),
    STR(u8),
    LOAD(u8),
    JMP(u8),
    JC(u8),
    JZ(u8),
    JO(u8),
    EXIT(u8),
    INC,
    DEC,
    ADD,
    SUB,
    NOP,
    OUT,
    CLN,
}
impl Instruction {
    pub fn execute(&self, cpu: &mut Cpu) -> Result<(), CpuError> {
        match self {
            Instruction::LDA(v) => {
                cpu.accumulator = cpu.memory.get(*v)?;
            }
            Instruction::STA(v) => {
                cpu.memory.set(*v, cpu.accumulator)?;
            }
            Instruction::INC => {
                if cpu.accumulator == 255 {
                    cpu.flags.zero = true;
                    cpu.flags.overflow = true
                }
                cpu.accumulator = cpu.accumulator.wrapping_add(1)
            }
            Instruction::DEC => {
                if cpu.accumulator == 1 {
                    cpu.flags.zero = true
                } else if cpu.accumulator == 0 {
                    cpu.flags.overflow = true
                }
                cpu.accumulator = cpu.accumulator.wrapping_sub(1)
            }
            Instruction::SETV(v) => cpu.user = *v,
            Instruction::SETA(v) => cpu.user = cpu.memory.get(*v)?,
            Instruction::STR(v) => cpu.memory.set(*v, cpu.user)?,
            Instruction::LOAD(v) => cpu.user = cpu.memory.get(*v)?,
            Instruction::ADD => {
                if let Some(acc) = cpu.accumulator.checked_add(cpu.user) {
                    cpu.accumulator = acc;
                } else {
                    cpu.flags.overflow = true;
                    cpu.accumulator = cpu.accumulator.wrapping_add(cpu.user)
                }
            }
            Instruction::SUB => cpu.accumulator = cpu.accumulator.wrapping_sub(cpu.user),
            Instruction::JMP(v) => cpu.ip = *v,
            Instruction::JC(v) => {
                if cpu.flags.any() {
                    cpu.ip = *v
                }
            }
            Instruction::JZ(v) => {
                if cpu.flags.zero {
                    cpu.ip = *v
                }
            }
            Instruction::JO(v) => {
                if cpu.flags.overflow {
                    cpu.ip = *v
                }
            }
            Instruction::OUT => println!("{}", cpu.accumulator),
            Instruction::NOP => (),
            Instruction::EXIT(ex_code) => return Err(CpuError::Exit(*ex_code)),
            Instruction::CLN => cpu.accumulator = cpu.user,
        }
        Ok(())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Instruction::EXIT(ex_code) => vec![0x11, *ex_code],
            Instruction::STA(v) => vec![0x02, *v],
            Instruction::LDA(v) => vec![0x01, *v],
            Instruction::SETV(v) => vec![0x05, *v],
            Instruction::SETA(v) => vec![0x06, *v],
            Instruction::STR(v) => vec![0x07, *v],
            Instruction::LOAD(v) => vec![0x08, *v],
            Instruction::JMP(v) => vec![0x0B, *v],
            Instruction::JC(v) => vec![0x0C, *v],
            Instruction::JZ(v) => vec![0x0E, *v],
            Instruction::JO(v) => vec![0x0F, *v],
            Instruction::INC => vec![0x03],
            Instruction::DEC => vec![0x04],
            Instruction::ADD => vec![0x09],
            Instruction::SUB => vec![0x0A],
            Instruction::OUT => vec![0x10],
            Instruction::NOP => vec![0x00],
            Instruction::CLN => vec![0x12],
        }
    }

    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Self::NOP),
            0x03 => Some(Self::INC),
            0x04 => Some(Self::DEC),
            0x09 => Some(Self::ADD),
            0x0A => Some(Self::SUB),
            0x10 => Some(Self::OUT),
            0x12 => Some(Self::CLN),
            _ => None,
        }
    }

    pub fn from_byte_and_arg(byte: u8, arg: u8) -> Result<Self, CpuError> {
        match byte {
            0x00 => Ok(Self::NOP),
            0x01 => Ok(Self::LDA(arg)),
            0x02 => Ok(Self::STA(arg)),
            0x05 => Ok(Self::SETV(arg)),
            0x06 => Ok(Self::SETA(arg)),
            0x07 => Ok(Self::STR(arg)),
            0x08 => Ok(Self::LOAD(arg)),
            0x0B => Ok(Self::JMP(arg)),
            0x0C => Ok(Self::JC(arg)),
            0x0E => Ok(Self::JZ(arg)),
            0x0F => Ok(Self::JO(arg)),
            0x11 => Ok(Self::EXIT(arg)),
            _ => Err(CpuError::MalformedInput(byte, arg)),
        }
    }
}
impl From<Instruction> for String {
    fn from(ins: Instruction) -> String {
        match ins {
            Instruction::LDA(v) => format!("LDA {}", v),
            Instruction::STA(v) => format!("STA {}", v),
            Instruction::INC => format!("INC"),
            Instruction::DEC => format!("DEC"),
            Instruction::SETV(v) => format!("SETV {}", v),
            Instruction::SETA(v) => format!("SETA {}", v),
            Instruction::STR(v) => format!("STR {}", v),
            Instruction::LOAD(v) => format!("LOAD {}", v),
            Instruction::ADD => format!("ADD"),
            Instruction::SUB => format!("SUB"),
            Instruction::JMP(v) => format!("JMP {}", v),
            Instruction::JC(v) => format!("JC {}", v),
            Instruction::JZ(v) => format!("JZ {}", v),
            Instruction::JO(v) => format!("JO {}", v),
            Instruction::OUT => format!("OUT"),
            Instruction::NOP => format!("NOP"),
            Instruction::EXIT(v) => format!("EXIT {}", v),
            Instruction::CLN => format!("CLN"),
        }
    }
}
impl TryFrom<String> for Instruction {
    // Ok(instruction_byte, arg_string)
    // Err(error_string)
    type Error = Result<(u8, String), String>;
    fn try_from(s: String) -> Result<Instruction, Self::Error> {
        let s = s.trim();
        if s.contains(" ") {
            let mut parts = s.split_whitespace();

            let instr_str = match parts.next() {
                Some(is) => is,
                None => unreachable!(),
            };
            let arg_str = match parts.next() {
                Some(arg) => arg,
                None => return Err(Err(format!("Expected argument for {:?}", instr_str))),
            };

            let arg = if arg_str.chars().all(|c| c.is_numeric()) {
                match arg_str.parse::<u8>() {
                    Ok(arg) => arg,
                    Err(e) => {
                        return Err(Err(format!(
                            "Encountered an error parsing {} as u8: {}",
                            arg_str, e
                        )))
                    }
                }
            } else if arg_str.contains("0x") {
                let arg_str = arg_str.trim_start_matches("0x");
                match u8::from_str_radix(arg_str, 16) {
                    Ok(num) => num,
                    Err(e) => {
                        return Err(Err(format!(
                            "Encountered an error parsing {} as u8: {}",
                            arg_str, e
                        )))
                    }
                }
            } else if arg_str.contains("0b") {
                let arg_str = arg_str.trim_start_matches("0b");
                match u8::from_str_radix(arg_str, 2) {
                    Ok(num) => num,
                    Err(e) => {
                        return Err(Err(format!(
                            "Encountered an error parsing {} as u8: {}",
                            arg_str, e
                        )))
                    }
                }
            } else {
                let instruction_byte = match instr_str.trim().to_uppercase().as_str() {
                    "EXIT" => 0x11,
                    "STA" => 0x02,
                    "LDA" => 0x01,
                    "SETV" => 0x05,
                    "SETA" => 0x06,
                    "STR" => 0x07,
                    "LOAD" => 0x08,
                    "JMP" => 0x0B,
                    "JC" => 0x0C,
                    "JZ" => 0x0E,
                    "JO" => 0x0F,
                    _ => 0,
                };
                return Err(Ok((instruction_byte, arg_str.to_owned())));
            };
            match instr_str.trim().to_uppercase().as_str() {
                "LDA" => Ok(Instruction::LDA(arg)),
                "STA" => Ok(Instruction::STA(arg)),
                "SETV" => Ok(Instruction::SETV(arg)),
                "SETA" => Ok(Instruction::SETA(arg)),
                "STR" => Ok(Instruction::STR(arg)),
                "LOAD" => Ok(Instruction::LOAD(arg)),
                "JMP" => Ok(Instruction::JMP(arg)),
                "JC" => Ok(Instruction::JC(arg)),
                "JZ" => Ok(Instruction::JZ(arg)),
                "JO" => Ok(Instruction::JO(arg)),
                "EXIT" => Ok(Instruction::EXIT(arg)),
                _ => Err(Err(format!("Unknown instruction {}", instr_str))),
            }
        } else {
            match s.trim().to_uppercase().as_str() {
                "INC" => Ok(Instruction::INC),
                "DEC" => Ok(Instruction::DEC),
                "ADD" => Ok(Instruction::ADD),
                "SUB" => Ok(Instruction::SUB),
                "NOP" => Ok(Instruction::NOP),
                "OUT" => Ok(Instruction::OUT),
                "CLN" => Ok(Instruction::CLN),
                _ => Err(Err(format!("Expected argument for {:?}", s))),
            }
        }
    }
}
