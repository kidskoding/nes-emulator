mod test_adc {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_adc_carry_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xFF;
        cpu.load_and_run(vec![0x69, 0x01, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status & 0b0000_0001, 1);
    }

    #[test]
    fn test_adc_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x00;
        cpu.load_and_run(vec![0x69, 0x00, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status & 0b0000_0010, 0b10);
    }

    #[test]
    fn test_adc_with_carry() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x50;
        cpu.status |= 0b0000_0001;
        cpu.load_and_run(vec![0x69, 0x50, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0xA1);
    }

    #[test]
    fn test_adc_overflow_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x50;
        cpu.load_and_run(vec![0x69, 0x50, 0x00]).unwrap();
        assert_eq!(cpu.status & 0b0100_0000, 0b0100_0000);
    }
}
