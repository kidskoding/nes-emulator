use crate::opcode::OpCode;
use lazy_static::lazy_static;

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
    NoneAddressing,
}

lazy_static! {
    pub static ref CPU_OPS_CODES: Vec<OpCode<'static>> = vec![
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
        OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, "LDA", 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0xb9, "LDA", 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0xa1, "LDA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xb1, "LDA", 2, 5, AddressingMode::Indirect_Y),

        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8d, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9d, "STA", 3, 5, AddressingMode::Absolute_X),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),
        OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),

        OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x6d, "ADC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x7d, "ADC", 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0x79, "ADC", 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x71, "ADC", 2, 5, AddressingMode::Indirect_Y),

        OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x55, "AND", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x65, "AND", 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0x75, "AND", 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0x29, "AND", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x25, "AND", 2, 5, AddressingMode::Indirect_Y),
    ];
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

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage  => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
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
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

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
        let value = self.mem_read(addr);

        self.register_a &= value;
        self.update_zero_and_negative_flags(self.register_a);
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
    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | lo
    }
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

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

            let operation = CPU_OPS_CODES.iter().find(|op| op.opcode == code);

            match operation {
                Some(opcode) => {
                    match opcode.name {
                        "LDA" => self.lda(&opcode.addressing_mode),
                        "STA" => self.sta(&opcode.addressing_mode),
                        "ADC" => self.adc(&opcode.addressing_mode),
                        "AND" => self.and(&opcode.addressing_mode),
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

mod test {
    use crate::cpu::CPU;

    mod test_lda {
        use crate::cpu::CPU;

        #[test]
        fn test_lda_immediate_load_data() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xA9, 0x05, 0x00]);

            assert_eq!(cpu.register_a, 0x05);
            assert_eq!(cpu.status & 0b0000_0010, 0);
            assert_eq!(cpu.status & 0b1000_0000, 0);
        }
        #[test]
        fn test_lda_sets_zero_flag() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xA9, 0x00, 0x00]);

            assert_eq!(cpu.register_a, 0x00);
            assert_eq!(cpu.status & 0b0000_0010, 0b10);
            assert_eq!(cpu.status & 0b1000_0000, 0);
        }
        #[test]
        fn test_lda_sets_negative_flag() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xA9, 0x80, 0x00]);

            assert_eq!(cpu.register_a, 0x80);
            assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
            assert_eq!(cpu.status & 0b0000_0010, 0);
        }
        #[test]
        fn test_lda_zero_page() {
            let mut cpu = CPU::new();
            cpu.mem_write(0x10, 0x42);
            cpu.load_and_run(vec![0xA5, 0x10, 0x00]);

            assert_eq!(cpu.register_a, 0x42);
            assert_eq!(cpu.status & 0b0000_0010, 0);
            assert_eq!(cpu.status & 0b1000_0000, 0);
        }
        #[test]
        fn test_lda_zero_page_x() {
            let mut cpu = CPU::new();
            cpu.register_x = 0x01;
            cpu.mem_write(0x11, 0x99);
            cpu.load_and_run(vec![0xB5, 0x10, 0x00]);

            assert_eq!(cpu.register_a, 0x99);
        }
        #[test]
        fn test_lda_absolute() {
            let mut cpu = CPU::new();
            cpu.mem_write(0x1234, 0x7F);
            cpu.load_and_run(vec![0xAD, 0x34, 0x12, 0x00]);

            assert_eq!(cpu.register_a, 0x7F);
        }
        #[test]
        fn test_lda_absolute_x() {
            let mut cpu = CPU::new();
            cpu.register_x = 0x01;
            cpu.mem_write(0x1235, 0x8A);
            cpu.load_and_run(vec![0xBD, 0x34, 0x12, 0x00]);

            assert_eq!(cpu.register_a, 0x8A);
        }
        #[test]
        fn test_lda_absolute_y() {
            let mut cpu = CPU::new();
            cpu.register_y = 0x01;
            cpu.mem_write(0x1235, 0xB7);
            cpu.load_and_run(vec![0xB9, 0x34, 0x12, 0x00]);

            assert_eq!(cpu.register_a, 0xB7);
        }
        #[test]
        fn test_lda_indirect_x() {
            let mut cpu = CPU::new();
            cpu.register_x = 0x04;
            cpu.mem_write(0x10, 0x00);
            cpu.mem_write(0x11, 0x20);
            cpu.mem_write(0x2000, 0xFE);
            cpu.load_and_run(vec![0xA1, 0x0C, 0x00]);

            assert_eq!(cpu.register_a, 0xFE);
        }
        #[test]
        fn test_lda_indirect_y() {
            let mut cpu = CPU::new();
            cpu.register_y = 0x01;
            cpu.mem_write(0x10, 0x00);
            cpu.mem_write(0x11, 0x20);
            cpu.mem_write(0x2001, 0x7E);
            cpu.load_and_run(vec![0xB1, 0x10, 0x00]);

            assert_eq!(cpu.register_a, 0x7E);
        }
    }
    mod test_tax {
        use crate::cpu::CPU;

        #[test]
        fn test_0xaa_tax_move_a_to_x() {
            let mut cpu = CPU::new();
            cpu.register_a = 10;
            cpu.load_and_run(vec![0xaa, 0x00]);

            assert_eq!(cpu.register_x, 10);
        }
    }
    mod test_inx {
        use crate::cpu::CPU;

        #[test]
        fn test_inx() {
            let mut cpu = CPU::new();
            cpu.register_x = 0x01;

            cpu.load_and_run(vec![0xe8, 0x00]);

            assert_eq!(cpu.register_x, 0x02);
            assert_eq!(cpu.status & 0b0000_0010, 0);
            assert_eq!(cpu.status & 0b1000_0000, 0);
        }
        #[test]
        fn test_inx_overflow() {
            let mut cpu = CPU::new();
            cpu.register_x = 0xff;
            cpu.load_and_run(vec![0xe8, 0xe8, 0x00]);

            assert_eq!(cpu.register_x, 1);
        }
    }
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1);
    }
    mod test_adc {
        use crate::cpu::CPU;

        #[test]
        fn test_adc_carry_flag() {
            let mut cpu = CPU::new();
            cpu.register_a = 0xFF;
            cpu.load_and_run(vec![0x69, 0x01, 0x00]);
            assert_eq!(cpu.register_a, 0x00);
            assert_eq!(cpu.status & 0b0000_0001, 1);
        }

        #[test]
        fn test_adc_zero_flag() {
            let mut cpu = CPU::new();
            cpu.register_a = 0x00;
            cpu.load_and_run(vec![0x69, 0x00, 0x00]);
            assert_eq!(cpu.register_a, 0x00);
            assert_eq!(cpu.status & 0b0000_0010, 0b10);
        }

        #[test]
        fn test_adc_with_carry() {
            let mut cpu = CPU::new();
            cpu.register_a = 0x50;
            cpu.status |= 0b0000_0001;
            cpu.load_and_run(vec![0x69, 0x50, 0x00]);
            assert_eq!(cpu.register_a, 0xA1);
        }

        #[test]
        fn test_adc_overflow_flag() {
            let mut cpu = CPU::new();
            cpu.register_a = 0x50;
            cpu.load_and_run(vec![0x69, 0x50, 0x00]);
            assert_eq!(cpu.status & 0b0100_0000, 0b0100_0000);
        }
    }
    mod test_and {
        use crate::cpu::CPU;

        #[test]
        fn test_and_basic() {
            let mut cpu = CPU::new();
            cpu.register_a = 0b1111_0000;
            cpu.load_and_run(vec![0x29, 0b0000_1111, 0x00]);
            assert_eq!(cpu.register_a, 0b0000_0000);
        }

        #[test]
        fn test_and_zero_flag() {
            let mut cpu = CPU::new();
            cpu.register_a = 0xFF;
            cpu.load_and_run(vec![0x29, 0x00, 0x00]);
            assert_eq!(cpu.register_a, 0x00);
            assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
        }

        #[test]
        fn test_and_negative_flag() {
            let mut cpu = CPU::new();
            cpu.register_a = 0xFF;
            cpu.load_and_run(vec![0x29, 0b1000_0000, 0x00]);
            assert_eq!(cpu.register_a, 0b1000_0000);
            assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
        }
    }
}
