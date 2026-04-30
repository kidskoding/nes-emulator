mod test_pla {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_pla() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x42;
        // PHA then PLA
        cpu.load_and_run(vec![0x48, 0x68, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0x42);
        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn test_pla_flags() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x00;
        cpu.load_and_run(vec![0x48, 0x68, 0x00]).unwrap();
        assert_eq!(cpu.status & 0b0000_0010, 0b10);
    }
}
