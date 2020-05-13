use std::io::Error;

#[derive(Debug)]
pub enum AssemblerError {
    IOError(Error),
    UnexpectedInstruction(String, usize),
    InstructionError(String),
    UndefinedLabel(String),
}
