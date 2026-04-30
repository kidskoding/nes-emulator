mod test_sbc {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_sbc_immediate() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        cpu.status = 1; // Carry set (no borrow)
        cpu.load_and_run(vec![0xE9, 0x03, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0x02);
        assert_eq!(cpu.status & 1, 1); // No borrow occurred
    }

    #[test]
    fn test_sbc_with_borrow() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        cpu.status = 0; // Carry clear (borrow)
        cpu.load_and_run(vec![0xE9, 0x03, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0x01); // 5 - 3 - 1 = 1
    }
}
