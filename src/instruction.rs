use crate::cpu::Cpu;
use crate::error::CpuError;
use crate::DEBUG;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    LDA(u8),
    STA(u8),
    INC,
    DEC,
    SETV(u8),
    SETA(u8),
    STR(u8),
    LOAD(u8),
    ADD,
    SUB,
    JMP(u8),
    JC(u8),
    JZ(u8),
    JO(u8),
    NOP,
    OUT,
    EXIT(u8),
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
