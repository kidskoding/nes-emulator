mod test_bvs {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_bvs_branch() {
        let mut cpu = CPU::new();
        cpu.status = 0b0100_0000;

        cpu.load_and_run(vec![0x70, 0x02]).unwrap();
        assert_eq!(cpu.program_counter, 0x8005);
    }

    #[test]
    fn test_bvs_no_branch() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0000;

        cpu.load_and_run(vec![0x70, 0x02]).unwrap();
        assert_eq!(cpu.program_counter, 0x8003);
    }
}