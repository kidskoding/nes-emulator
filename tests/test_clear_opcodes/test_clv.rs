mod test_clv {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_clv() {
        let mut cpu = CPU::new();
        cpu.status = 0b0100_0000;
        cpu.load_and_run(vec![0xB8]).unwrap();
        assert_eq!(cpu.status, 0b0000_0000);
    }
}