mod test_clc {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_clc() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0001;
        cpu.load_and_run(vec![0x18]).unwrap();
        assert_eq!(cpu.status, 0b0000_0000);
    }
}