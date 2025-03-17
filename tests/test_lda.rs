mod test_lda {
    use nes_emulator::cpu::CPU;

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
