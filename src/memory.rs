use crate::error::CpuError;
use crate::MEMORY_SIZE;

pub struct Memory {
    internal: [u8; MEMORY_SIZE],
}
impl Memory {
    fn is_valid_idx(idx: u8) -> Result<(), CpuError> {
        if idx as usize >= MEMORY_SIZE {
            Err(CpuError::MemoryError(idx))
        } else {
            Ok(())
        }
    }

    pub fn new() -> Self {
        Self {
            internal: [0; MEMORY_SIZE],
        }
    }

    pub fn new_with_instructions(instr: &Vec<u8>) -> Self {
        let mut internal = [0; MEMORY_SIZE];
        if instr.len() >= 0xFF {
            panic!("Memory overflow when initializing memory with instructions")
        }
        Self::is_valid_idx(instr.len() as u8)
            .expect("Memory overflow when initializing memory with instructions");
        unsafe {
            std::ptr::copy(instr.as_ptr(), &mut internal[0] as *mut u8, instr.len());
        }

        Self { internal }
    }

    pub fn set(&mut self, idx: u8, v: u8) -> Result<(), CpuError> {
        Self::is_valid_idx(idx)?;
        self.internal[idx as usize] = v;
        Ok(())
    }

    pub fn get(&self, idx: u8) -> Result<u8, CpuError> {
        Self::is_valid_idx(idx)?;
        Ok(self.internal[idx as usize])
    }
}
impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Memory {{\n\t\t{}\n\t}}", {
            let mut lines: Vec<String> = Vec::new();

            let even_lines = MEMORY_SIZE % 8 == 0;
            let num_lines = ((MEMORY_SIZE - (MEMORY_SIZE % 8)) / 8) - 1;

            for offs in (0..num_lines).map(|offs| offs * 8) {
                lines.push(
                    self.internal[offs..offs + 8]
                        .iter()
                        .map(|byte| format!("{:02X}", byte))
                        .collect::<Vec<String>>()
                        .join("  "),
                );
            }
            if !even_lines {
                lines.push(
                    self.internal[num_lines + 1 * 8..]
                        .iter()
                        .map(|byte| format!("{:02X}", byte))
                        .collect::<Vec<String>>()
                        .join("  "),
                );
            }

            lines.join(",\n\t\t")
        })
    }
}
