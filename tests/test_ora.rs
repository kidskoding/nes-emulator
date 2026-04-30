mod test_ora {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_ora_immediate() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1010_1010;
        cpu.load_and_run(vec![0x09, 0b0101_0101, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0b1111_1111);
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
    }

    #[test]
    fn test_ora_zero_page() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0000_1111;
        cpu.mem_write(0x10, 0b1111_0000);
        cpu.load_and_run(vec![0x05, 0x10, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0b1111_1111);
    }
}
