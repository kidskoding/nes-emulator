mod test_bcc {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_bcc_no_branch() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0001;

        cpu.load_and_run(vec![0x90, 0x05, 0x0a]);

        println!("{:x}", cpu.program_counter as u16);
        assert_eq!(cpu.program_counter, 0x8004);
    }

    #[test]
    fn test_bcc_branch() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0000;

        cpu.load_and_run(vec![0x90, 0x05, 0x0a]);
        println!("{:x}", cpu.program_counter as u16);
        assert_eq!(cpu.program_counter, 0x8009);
    }
}
