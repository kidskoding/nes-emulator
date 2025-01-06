use crate::cpu::AddressingMode;

pub struct OpCode<'a> {
    pub opcode: u8,
    pub name: &'a str,
    pub bytes: u8,
    pub cycles: u8,
    pub addressing_mode: AddressingMode,
}
impl<'a> OpCode<'a> {
    pub fn new(opcode: u8,
               name: &'a str,
               bytes: u8,
               cycles: u8,
               addressing_mode: AddressingMode
    ) -> OpCode<'a> {
        OpCode {
            opcode,
            name,
            bytes,
            cycles,
            addressing_mode
        }
    }
}
