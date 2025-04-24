mod test_bcc {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_bcc_no_branch() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0001;

        cpu.load_and_run(vec![0x90, 0x05]).unwrap();
        assert_eq!(cpu.program_counter, 0x8003);
    }

    #[test]
    fn test_bcc_branch() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0000;

        cpu.load_and_run(vec![0x90, 0x05]).unwrap();
        assert_eq!(cpu.program_counter, 0x8008);
    }
}
