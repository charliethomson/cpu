#[derive(Default)]
pub struct Flags {
    pub zero: bool,
    pub overflow: bool,
    pub clear: bool,
}
impl Flags {
    pub fn any(&self) -> bool {
        self.zero || self.overflow
    }

    pub fn clear_flags(&mut self) {
        self.zero = false;
        self.overflow = false;
        self.clear = false;
    }
}
impl std::fmt::Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Flags {{\n\t\tzero: {};\n\t\toverflow: {};\n\t\tclear: {};\n\t}}",
            self.zero, self.overflow, self.clear,
        )
    }
}
