mod test_jmp {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_jmp_absolute() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0x4C, 0x34, 0x12, 0x00]).unwrap();
        assert_eq!(cpu.program_counter, 0x1235);
    }

    #[test]
    fn test_jmp_indirect() {
        let mut cpu = CPU::new();

        cpu.mem_write(0x0010, 0x78);
        cpu.mem_write(0x0011, 0x56);

        cpu.load_and_run(vec![0x6C, 0x10, 0x00, 0x00]).unwrap();
        assert_eq!(cpu.program_counter, 0x5679);
    }
}