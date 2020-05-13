pub const MEMORY_SIZE: usize = 255;
pub const DEBUG: bool = false;

pub(crate) mod asm;
pub(crate) mod cpu;
pub(crate) mod error;
pub(crate) mod flags;
pub(crate) mod instruction;
pub(crate) mod memory;

fn main() -> Result<(), error::CpuError> {
    let mut cpu = cpu::Cpu::from_binary("./tests/fib.bin").unwrap();
    cpu.run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const NOP: u8 = 0x00;
    pub const LDA: u8 = 0x01;
    pub const STA: u8 = 0x02;
    pub const INC: u8 = 0x03;
    pub const DEC: u8 = 0x04;
    pub const SETV: u8 = 0x05;
    pub const SETA: u8 = 0x06;
    pub const STR: u8 = 0x07;
    pub const LOAD: u8 = 0x08;
    pub const ADD: u8 = 0x09;
    pub const SUB: u8 = 0x0A;
    pub const JMP: u8 = 0x0B;
    pub const JC: u8 = 0x0C;
    pub const JZ: u8 = 0x0E;
    pub const JO: u8 = 0x0F;
    pub const OUT: u8 = 0x10;
    pub const EXIT: u8 = 0x11;
    pub const CLN: u8 = 0x12;

    #[test]
    fn test_acc_ops() {
        let mut cpu = cpu::Cpu::new();
        cpu.memory = memory::Memory::new_with_instructions(&vec![
            SETV, 10, STR, 0x40, LDA, 0x40, SETV, 1, ADD, OUT, EXIT, 0,
        ]);

        match cpu.run() {
            Err(error::CpuError::Exit(code)) => assert_eq!(code, 0),
            Err(e) => panic!("{:?}", e),
            _ => (),
        };
        assert_eq!(11, cpu.accumulator);
    }

    #[test]
    fn test_load_store() {
        let mut cpu = cpu::Cpu::new();
        println!("Expected output: 0, 10, 11");
        cpu.memory = memory::Memory::new_with_instructions(&vec![
            OUT, // -> 0
            SETV, 10, STR, 0x40, LDA, 0x40, OUT, // -> 10
            INC, STA, 0x40, LOAD, 0x40, CLN, OUT, // -> 11
            EXIT, 0,
        ]);
        assert_eq!(cpu.run(), Err(error::CpuError::Exit(0)));
    }

    #[test]
    fn test_fib() {
        let mut cpu = cpu::Cpu::new();
        cpu.memory = memory::Memory::new_with_instructions(&vec![
            // init:
            SETV, 0, STR, 0x40, // x = 0
            STR, 0x42, // z = 0
            SETV, 1, STR, 0x41, // y = 1
            // loop

            // print z
            LDA, 0x42, OUT, // z = x + y
            LDA, 0x40, // load x into acc
            LOAD, 0x41, // load y into usr
            ADD,  // add y to acc (x) -> acc = x + y
            JO, 31, // exit if overflow
            STA, 0x42, // store acc in z
            // x = y
            LDA, 0x41, // load y into acc
            STA, 0x40, // store y in x
            // y = z
            LDA, 0x42, // load z into acc
            STA, 0x41, // store z in y
            // while z < 255
            JMP, 9, // reenter the loop otherwise
            // exit_good:
            EXIT, 1,
        ]);

        match cpu.run() {
            Err(error::CpuError::Exit(1)) => eprintln!("Exited correctly"),
            Err(error::CpuError::Exit(code)) => panic!("Exited with code {}", code),
            Err(e) => panic!("{:?}", e),
            _ => (),
        };
    }

    #[test]
    fn test_assemble_and_run() {
        let file_handle = std::fs::File::open("./tests/fib.as").unwrap();
        let mut assembler = asm::assembler::Assembler::new(file_handle);
        let nbytes = match assembler.parse() {
            Ok(nbytes) => nbytes,
            Err(e) => panic!("Encountered an error parsing \"./tests/fib.as\": {:?}", e),
        };

        eprintln!("Assembled {} bytes from \"./tests/fib.as\"", nbytes);
        let mut cpu = cpu::Cpu::new();
        cpu.memory = memory::Memory::new_with_instructions(&assembler.get_output());

        match cpu.run() {
            Err(error::CpuError::Exit(exit_code)) => {
                if exit_code == 1 {
                    eprintln!("Exited correctly");
                    return;
                } else {
                    unreachable!();
                }
            }
            Err(e) => panic!("{:?}", e),
            Ok(()) => return,
        }
    }
}
