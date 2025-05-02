mod test_eor {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_eor_zero_page_x() {
        let mut cpu = CPU::new();

        cpu.register_a = 0x33;
        cpu.register_x = 0x05;
        cpu.mem_write(0x0025, 0x55);

        cpu.load_and_run(vec![0x55, 0x20, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0x66);
    }
}
