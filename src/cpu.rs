use crate::opcode::CPU_OPCODES;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
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
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                Some(addr)
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                Some(addr)
            }
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                Some(addr)
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                Some(addr)
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                Some((hi as u16) << 8 | (lo as u16))
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                Some(deref)
            }
            AddressingMode::Accumulator => None,
            AddressingMode::Relative => None,
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr.unwrap());

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr.unwrap(), self.register_a);
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
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr.unwrap());

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
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr.unwrap());

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

        if self.status & 0b0000_0001 == 0 {
            self.program_counter = self.program_counter
                .wrapping_add(1)
                .wrapping_add(displacement as u16);
        }
    }
    fn bcs(&mut self) {
        let displacement: i8 = self.mem_read(self.program_counter) as i8;

        if self.status & 0b0000_0001 != 0 {
            self.program_counter = self.program_counter
                .wrapping_add(1)
                .wrapping_add(displacement as u16);
        }
    }
    fn beq(&mut self) {
        let displacement: i8 = self.mem_read(self.program_counter) as i8;

        if self.status & 0b0000_0010 != 0 {
            self.program_counter = self.program_counter
                .wrapping_add(1)
                .wrapping_add(displacement as u16);
        }
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

    fn mem_read(&self, addr: u16) -> u8 {
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
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.run();
    }

    pub fn run(&mut self) {
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let operation = CPU_OPCODES.iter().find(|op| op.opcode == code);

            match operation {
                Some(opcode) => {
                    match opcode.name {
                        "LDA" => self.lda(&opcode.addressing_mode),
                        "STA" => self.sta(&opcode.addressing_mode),
                        "ADC" => self.adc(&opcode.addressing_mode),
                        "AND" => self.and(&opcode.addressing_mode),
                        "ASL" => self.asl(&opcode.addressing_mode),
                        "BCC" => self.bcc(),
                        "BCS" => self.bcs(),
                        "BEQ" => self.beq(),
                        "TAX" => self.tax(),
                        "INX" => self.inx(),
                        "BRK" => return,
                        _ => panic!("CPU instruction {} not implemented", opcode.name),
                    }
                    self.program_counter += (opcode.bytes - 1) as u16;
                }
                None => {
                    panic!("Unknown opcode {:#x}", code);
                }
            }
        }
    }
}
