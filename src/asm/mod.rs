pub(crate) mod assembler;
pub(crate) mod error;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

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
    fn test_assembler_output() {
        let expected = vec![
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
        ];

        let file_handle = File::open("./tests/fib.as").unwrap();
        let mut assembler = assembler::Assembler::new(file_handle);
        eprintln!("Assembled {} bytes", assembler.parse().unwrap());
        let actual = assembler.get_output();

        assert_eq!(expected, actual);
    }
}
