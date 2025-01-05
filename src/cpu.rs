pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if self.register_a == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }

        if self.register_a & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.register_a = param;
                    self.update_zero_and_negative_flags(self.register_a);
                }
                0xAA => {
                    self.register_x = self.register_a;
                    self.update_zero_and_negative_flags(self.register_x);
                }
                0xE8 => {
                    self.register_x = self.register_x.wrapping_add(1);
                    self.update_zero_and_negative_flags(self.register_x);
                }
                0x00 => return,
                _ => todo!()
            }
        }
    }
}

mod test {
    use crate::cpu::CPU;

    mod test_0xa9 {
        use crate::cpu::CPU;

        #[test]
        fn test_0xa9_lda_immediate_load_data() {
            let mut cpu = CPU::new();
            cpu.interpret(vec![0xa9, 0x05, 0x00]);
            assert_eq!(cpu.register_a, 0x05);
            assert_eq!(cpu.status & 0b0000_0010, 0b00);
            assert_eq!(cpu.status & 0b1000_0000, 0);
        }

        #[test]
        fn test_0xa9_lda_zero_flag() {
            let mut cpu = CPU::new();
            cpu.interpret(vec![0xa9, 0x00, 0x00]);
            assert_eq!(cpu.status & 0b0000_0010, 0b10);
        }

        #[test]
        fn test_0xa9_lda_negative_flag() {
            let mut cpu = CPU::new();
            cpu.interpret(vec![0xa9, 0x80, 0x00]);
            assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
        }
    }
    mod test_0xaa {
        use crate::cpu::CPU;

        #[test]
        fn test_0xaa_tax_move_a_to_x() {
            let mut cpu = CPU::new();
            cpu.register_a = 10;
            cpu.interpret(vec![0xaa, 0x00]);

            assert_eq!(cpu.register_x, 10)
        }
    }
    mod test_0xe8 {
        use crate::cpu::CPU;

        #[test]
        fn test_inx_overflow() {
            let mut cpu = CPU::new();
            cpu.register_x = 0xff;
            cpu.interpret(vec![0xe8, 0xe8, 0x00]);

            assert_eq!(cpu.register_x, 1)
        }
    }
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }
}
