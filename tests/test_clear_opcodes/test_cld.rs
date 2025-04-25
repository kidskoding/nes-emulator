mod test_cld {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_cld() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_1000;
        cpu.load_and_run(vec![0xD8]).unwrap();
        assert_eq!(cpu.status, 0b0000_0000);
    }
}