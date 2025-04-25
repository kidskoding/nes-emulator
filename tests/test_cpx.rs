mod test_cpx {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_cpx() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xC0, 0x05, 0xC0, 0x03, 0xC0, 0x07]).unwrap();

        assert_eq!(cpu.program_counter, 0x8007);
    }
}