mod test_and {
    use nes_emulator::cpu::CPU;

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
