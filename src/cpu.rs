use crate::{error::CPUError, opcode::CPU_OPCODES};

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Implied,
    Accumulator,
    Relative,
    NoneAddressing,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF]
        }
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> Option<u16> {
        match mode {
            AddressingMode::Immediate => Some(self.program_counter),
            AddressingMode::ZeroPage  => Some(self.mem_read(self.program_counter) as u16),
            AddressingMode::Absolute => Some(self.mem_read_u16(self.program_counter)),
            AddressingMode::ZeroPageX => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                Some(addr)
            }
            AddressingMode::ZeroPageY => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                Some(addr)
            }
            AddressingMode::AbsoluteX => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                Some(addr)
            }
            AddressingMode::AbsoluteY => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                Some(addr)
            }
            AddressingMode::IndirectX => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                Some((hi as u16) << 8 | (lo as u16))
            }
            AddressingMode::IndirectY => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                Some(deref)
            }
            AddressingMode::Accumulator 
                | AddressingMode::Relative 
                | AddressingMode::Implied => None,
            AddressingMode::NoneAddressing => panic!("{}", CPUError::InvalidAddressingMode(mode)),
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode).unwrap();
        let value: u8 = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode).unwrap();
        self.mem_write(addr, self.register_a);
    }
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode).unwrap();
        let value: u8 = self.mem_read(addr);

        let result = self.register_a as u16
            + value as u16
            + (self.status & 0b0000_0001) as u16;

        if result > 0xFF {
            self.status |= 0b0000_0001;
        } else {
            self.status &= 0b1111_1110;
        }

        let a = self.register_a;
        let result8 = (result & 0xFF) as u8;
        if (a ^ value) & 0x80 == 0 && (a ^ result8) & 0x80 != 0 {
            self.status |= 0b0100_0000;
        } else {
            self.status &= 0b1011_1111;
        }

        self.register_a = result8;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode).unwrap();
        let value = self.mem_read(addr);

        self.register_a &= value;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn asl(&mut self, mode: &AddressingMode) {
        if let Some(addr) = self.get_operand_address(mode) {
            let mut value = self.mem_read(addr);
            self.status = (self.status & 0b1111_1110) | ((value >> 7) & 1);
            value <<= 1;
            self.mem_write(addr, value);
            self.update_zero_and_negative_flags(value);
        } else {
            self.status = (self.status & 0b1111_1110) | ((self.register_a >> 7) & 1);
            self.register_a <<= 1;
            self.update_zero_and_negative_flags(self.register_a);
        }
    }
    fn bcc(&mut self) {
        let displacement: i8 = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if self.status & 0b0000_0001 == 0 {
            self.program_counter = 
                self.program_counter.wrapping_add(displacement as u16)
        }
    }
    fn bcs(&mut self) {
        let displacement: i8 = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if self.status & 0b0000_0001 != 0 {
            self.program_counter = 
                self.program_counter.wrapping_add(displacement as u16);
        }
    }
    fn beq(&mut self) {
        let displacement: i8 = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if self.status & 0b0000_0010 != 0 {
            self.program_counter = 
                self.program_counter.wrapping_add(displacement as u16)
        }
    }
    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode).unwrap();
        let value = self.mem_read(addr);

        let result = self.register_a & value;

        if result == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
    
        if value & 0b0100_0000 != 0 {
            self.status |= 0b0100_0000;
        } else {
            self.status &= 0b1011_1111;
        }
    
        if value & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }
    fn bmi(&mut self) {
        let displacement: i8 = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if self.status & 0b1000_0000 != 0 {
            self.program_counter = 
                self.program_counter.wrapping_add(displacement as u16)
        }
    }
    fn bne(&mut self) {
        let displacement: i8 = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if self.status & 0b0000_0010 == 0 {
            self.program_counter = 
                self.program_counter.wrapping_add(displacement as u16)
        }
    }
    fn bpl(&mut self) {
        let displacement: i8 = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if self.status & 0b1000_0000 == 0 {
            self.program_counter = 
                self.program_counter.wrapping_add(displacement as u16)
        }
    }
    fn bvc(&mut self) {
        let displacement: i8 = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if self.status & 0b0100_0000 == 0 {
            self.program_counter = 
                self.program_counter.wrapping_add(displacement as u16)
        }
    }
    fn bvs(&mut self) {
        let displacement: i8 = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if self.status & 0b0100_0000 != 0 {
            self.program_counter = 
                self.program_counter.wrapping_add(displacement as u16)
        }
    }
    fn cmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode).unwrap();
        let value = self.mem_read(addr);

        let result = self.register_a.wrapping_sub(value);

        if self.register_a >= value {
            self.status |= 0b0000_0001;
        } else {
            self.status &= 0b1111_1110;
        }

        self.update_zero_and_negative_flags(result);
    }
    fn cpx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode).unwrap();
        let value = self.mem_read(addr);

        let result = self.register_x.wrapping_sub(value);

        if self.register_x >= value {
            self.status |= 0b0000_0001;
        } else {
            self.status &= 0b1111_1110;
        }

        self.update_zero_and_negative_flags(result);
    }
    fn cpy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode).unwrap();
        let value = self.mem_read(addr);

        let result = self.register_y.wrapping_sub(value);

        if self.register_y >= value {
            self.status |= 0b0000_0001;
        } else {
            self.status &= 0b1111_1110;
        }

        self.update_zero_and_negative_flags(result);
    }
    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode).unwrap();
        let value = self.mem_read(addr);
        let result = value.wrapping_sub(1);
        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
    }
    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }
    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode).unwrap();
        let value = self.mem_read(addr);
        self.register_a ^= value;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode).unwrap();
        let mut value = self.mem_read(addr);
        value = value.wrapping_add(1);
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }
    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y); 
    }


    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }
    /* fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    } */

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x8000;
    }
    pub fn load_and_run(&mut self, program: Vec<u8>) -> Result<(), CPUError> {
        self.load(program);
        self.run()?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), CPUError> {
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let operation = CPU_OPCODES
                .iter()
                .find(|op| op.opcode == code);

            if let Some(opcode) = operation {
                match opcode.name {
                    "LDA" => self.lda(&opcode.addressing_mode),
                    "STA" => self.sta(&opcode.addressing_mode),
                    "ADC" => self.adc(&opcode.addressing_mode),
                    "AND" => self.and(&opcode.addressing_mode),
                    "ASL" => self.asl(&opcode.addressing_mode),
                    "BCC" => self.bcc(),
                    "BCS" => self.bcs(),
                    "BEQ" => self.beq(),
                    "BIT" => self.bit(&opcode.addressing_mode),
                    "BMI" => self.bmi(),
                    "BNE" => self.bne(),
                    "BPL" => self.bpl(),
                    "BRK" => return Ok(()),
                    "BVC" => self.bvc(),
                    "BVS" => self.bvs(),
                    "CLC" => self.status &= 0b1111_1110,
                    "CLD" => self.status &= 0b1111_0111,
                    "CLI" => self.status &= 0b1111_1011,
                    "CLV" => self.status &= 0b1011_1111,
                    "CMP" => self.cmp(&opcode.addressing_mode),
                    "CPX" => self.cpx(&opcode.addressing_mode),
                    "CPY" => self.cpy(&opcode.addressing_mode),
                    "DEC" => self.dec(&opcode.addressing_mode),
                    "DEX" => self.dex(),
                    "DEY" => self.dey(),
                    "EOR" => self.eor(&opcode.addressing_mode),
                    "INC" => self.inc(&opcode.addressing_mode),
                    "TAX" => self.tax(),
                    "INX" => self.inx(),
                    "INY" => self.iny(),
                    _ => return Err(CPUError::UnimplementedInstruction(opcode.name.to_string())),
                }
                if opcode.addressing_mode != AddressingMode::Relative {
                    self.program_counter += (opcode.bytes - 1) as u16;
                }
            } else {
                return Err(CPUError::UnknownOpcode(code));
            }
        }
    }
}
