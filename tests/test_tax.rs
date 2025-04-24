mod test_tax {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_tax() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.load_and_run(vec![0xaa, 0x00]).unwrap();

        assert_eq!(cpu.register_x, 10);
    }
}
