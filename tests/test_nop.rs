mod test_nop {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_nop() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x42;
        cpu.status = 0x12;
        cpu.load_and_run(vec![0xEA, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0x42);
        assert_eq!(cpu.status, 0x12);
        assert_eq!(cpu.program_counter, 0x8002);
    }
}
