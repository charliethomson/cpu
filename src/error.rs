#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CpuError {
    MemoryError(u8),
    Exit(u8),
    VOverflow,
    AOverflow,
    MalformedInput(u8, u8),
}
