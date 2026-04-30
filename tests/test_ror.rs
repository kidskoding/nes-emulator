mod test_ror {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_ror_accumulator() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_0001;
        cpu.status = 1; // Carry set
        cpu.load_and_run(vec![0x6A, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0b1000_0000);
        assert_eq!(cpu.status & 1, 1); // Old bit 0 was 1, so Carry still set
    }

    #[test]
    fn test_ror_zero_page() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b1000_0000);
        cpu.status = 0; // Carry clear
        cpu.load_and_run(vec![0x66, 0x10, 0x00]).unwrap();
        assert_eq!(cpu.mem_read(0x10), 0b0100_0000);
        assert_eq!(cpu.status & 1, 0);
    }
}
