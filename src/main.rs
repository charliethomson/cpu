
pub mod cpu;
pub mod cell;
pub mod instruction;
pub mod util;

pub const SUPPRESS_WARNINGS: bool = false;

fn main() -> Result<(), String> {
    let processor = match cpu::Cpu::new().with_file("./tests/echo.as") {
        Ok(cpu) => cpu,
        Err(e) => return Err(format!("Encountered an error: {}", e))
    };
    processor.run()?;
    Ok(())
}

