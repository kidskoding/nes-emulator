mod test_cli {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_cli() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0100;
        cpu.load_and_run(vec![0x58]).unwrap();
        assert_eq!(cpu.status, 0b0000_0000);
    }
}