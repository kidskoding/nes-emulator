mod test_lsr {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_lsr_accumulator() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        cpu.load_and_run(vec![0x4A, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0x02);
        assert_eq!(cpu.status & 0b0000_0001, 1);
    }

    #[test]
    fn test_lsr_zero_page() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x01);
        cpu.load_and_run(vec![0x46, 0x10, 0x00]).unwrap();
        assert_eq!(cpu.mem_read(0x10), 0x00);
        assert_eq!(cpu.status & 0b0000_0001, 1);
        assert_eq!(cpu.status & 0b0000_0010, 0b10);
    }

    #[test]
    fn test_lsr_absolute() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1234, 0x80);
        cpu.load_and_run(vec![0x4E, 0x34, 0x12, 0x00]).unwrap();
        assert_eq!(cpu.mem_read(0x1234), 0x40);
        assert_eq!(cpu.status & 0b0000_0001, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0); // Negative flag always 0 for LSR
    }
}
