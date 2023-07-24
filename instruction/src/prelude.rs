use crate::Result;

/// Instruction
pub trait Instruction<M> {
    /// Number of Register
    const REGISTER_NUMBER: usize;

    /// Register type, usually u32 or u64
    type Register;

    /// Execute an anstruction.
    fn execute(self, pc: &mut Self::Register, regs: &mut [Self::Register]) -> Result<()>;
}

/// Readable Linear Memory
pub trait Memory {
    type Register;

    /// Memory length
    fn length(&self) -> Self::Register;

    /// Load data from memory
    fn load(&self, pos: Self::Register, length: u8) -> &[u8];
}