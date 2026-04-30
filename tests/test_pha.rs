mod test_pha {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_pha() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x42;
        cpu.load_and_run(vec![0x48, 0x00]).unwrap();
        assert_eq!(cpu.stack_pointer, 0xFE);
        assert_eq!(cpu.mem_read(0x01FF), 0x42);
    }
}
