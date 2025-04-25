use std::fmt;
use crate::cpu::AddressingMode;

#[derive(Debug)]
pub enum CPUError<'a> {
    UnknownOpcode(u8),
    UnimplementedInstruction(String),
    InvalidAddressingMode(&'a AddressingMode),
}

impl fmt::Display for CPUError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CPUError::UnknownOpcode(opcode) => 
                write!(f, "unknown opcode: 0x{:02X}", opcode),
            CPUError::UnimplementedInstruction(name) => 
                write!(f, "CPU instruction {} not implemented", name),
            CPUError::InvalidAddressingMode(mode) => 
                write!(f, "addressing mode {:?} is not supported", mode),
        }
    }
}
