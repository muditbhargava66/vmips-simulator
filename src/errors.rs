use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SimulatorError {
    // Memory errors
    MemoryOutOfBounds(u32),
    MemoryMisaligned(u32),
    AddressOverflow,

    // Branch errors
    InvalidBranchTarget(u32),

    // Execution errors
    InvalidInstruction(u32),
    DivisionByZero,

    // System errors
    IoError(std::io::Error),

    // Other errors
    UnimplementedFeature(String),
}

impl fmt::Display for SimulatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimulatorError::MemoryOutOfBounds(addr) => {
                write!(f, "Memory access out of bounds: 0x{:08X}", addr)
            },
            SimulatorError::MemoryMisaligned(addr) => {
                write!(f, "Misaligned memory access: 0x{:08X}", addr)
            },
            SimulatorError::AddressOverflow => {
                write!(f, "Address calculation resulted in overflow")
            },
            SimulatorError::InvalidBranchTarget(addr) => {
                write!(f, "Invalid branch target address: 0x{:08X}", addr)
            },
            SimulatorError::InvalidInstruction(instr) => {
                write!(f, "Invalid instruction: 0x{:08X}", instr)
            },
            SimulatorError::DivisionByZero => write!(f, "Division by zero"),
            SimulatorError::IoError(err) => write!(f, "I/O error: {}", err),
            SimulatorError::UnimplementedFeature(feature) => {
                write!(f, "Unimplemented feature: {}", feature)
            },
        }
    }
}

impl Error for SimulatorError {}

impl From<std::io::Error> for SimulatorError {
    fn from(error: std::io::Error) -> Self {
        SimulatorError::IoError(error)
    }
}
