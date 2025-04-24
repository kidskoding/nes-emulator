mod bcs_test {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_bcs_branch() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0000;

        cpu.load_and_run(vec![0x90, 0x05, 0x0a]).unwrap();
        assert_eq!(cpu.program_counter, 0x8008);
    }

    #[test]
    fn test_bcs_no_branch() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0001;

        cpu.load_and_run(vec![0x90, 0x05, 0x0a]).unwrap();
        assert_eq!(cpu.program_counter, 0x8004);
    }
}
