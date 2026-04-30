mod test_plp {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_plp() {
        let mut cpu = CPU::new();
        // PHP sets bits 4 and 5, so we use PHA to push custom status
        cpu.register_a = 0b1000_0001;
        cpu.load_and_run(vec![0x48, 0x28, 0x00]).unwrap();
        assert_eq!(cpu.status & 0b1100_1111, 0b1000_0001);
    }
}
