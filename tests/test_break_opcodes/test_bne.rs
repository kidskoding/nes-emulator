mod test_bne {
    #[test]
    fn test_bne_branch() {
        let mut cpu = nes_emulator::cpu::CPU::new();
        cpu.status = 0b0000_0000;

        cpu.load_and_run(vec![0xD0, 0x05]).unwrap();
        assert_eq!(cpu.program_counter, 0x8008);
    }

    #[test]
    fn test_bne_no_branch() {
        let mut cpu = nes_emulator::cpu::CPU::new();
        cpu.status = 0b0000_0010;

        cpu.load_and_run(vec![0xD0, 0x05]).unwrap();
        assert_eq!(cpu.program_counter, 0x8003);
    }
}
