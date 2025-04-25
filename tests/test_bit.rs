mod test_bit {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_bit_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xFF;
        cpu.mem_write(0x0000, 0x00);
        
        cpu.load_and_run(vec![0x24, 0x00, 0x00]).unwrap();
        
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
    }

    #[test]
    fn test_bit_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.mem_write(0x0000, 0x80);
        
        cpu.load_and_run(vec![0x24, 0x00, 0x00]).unwrap();
        
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
    }

    #[test]
    fn test_bit_overflow_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.mem_write(0x0000, 0x40);
        
        cpu.load_and_run(vec![0x24, 0x00, 0x00]).unwrap();
        
        assert_eq!(cpu.status & 0b0100_0000, 0b0100_0000);
    }

    #[test]
    fn test_bit_no_flags() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x01;
        cpu.mem_write(0x0000, 0x01);
        
        cpu.load_and_run(vec![0x24, 0x00, 0x00]).unwrap();
        
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0);
        assert_eq!(cpu.status & 0b0100_0000, 0);
    }

    #[test]
    fn test_bit_all_flags() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x00;
        cpu.mem_write(0x0000, 0xC0);
        
        cpu.load_and_run(vec![0x24, 0x00, 0x00]).unwrap();
        
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
        assert_eq!(cpu.status & 0b0100_0000, 0b0100_0000);
    }
}