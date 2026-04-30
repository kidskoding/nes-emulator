mod test_ldy {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_ldy_immediate() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA0, 0x05, 0x00]).unwrap();
        assert_eq!(cpu.register_y, 0x05);
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }

    #[test]
    fn test_ldy_zero_page() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x42);
        cpu.load_and_run(vec![0xA4, 0x10, 0x00]).unwrap();
        assert_eq!(cpu.register_y, 0x42);
    }

    #[test]
    fn test_ldy_zero_page_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;
        cpu.mem_write(0x11, 0x99);
        cpu.load_and_run(vec![0xB4, 0x10, 0x00]).unwrap();
        assert_eq!(cpu.register_y, 0x99);
    }

    #[test]
    fn test_ldy_absolute() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1234, 0x7F);
        cpu.load_and_run(vec![0xAC, 0x34, 0x12, 0x00]).unwrap();
        assert_eq!(cpu.register_y, 0x7F);
    }

    #[test]
    fn test_ldy_absolute_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;
        cpu.mem_write(0x1235, 0x8A);
        cpu.load_and_run(vec![0xBC, 0x34, 0x12, 0x00]).unwrap();
        assert_eq!(cpu.register_y, 0x8A);
    }

    #[test]
    fn test_ldy_sets_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA0, 0x00, 0x00]).unwrap();
        assert_eq!(cpu.status & 0b0000_0010, 0b10);
    }

    #[test]
    fn test_ldy_sets_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA0, 0x80, 0x00]).unwrap();
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
    }
}
