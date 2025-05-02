mod test_iny {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_iny() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x01;

        cpu.load_and_run(vec![0xc8]).unwrap();

        assert_eq!(cpu.register_y, 0x02);
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }
    #[test]
    fn test_iny_overflow() {
        let mut cpu = CPU::new();
        cpu.register_y = 0xff;
        cpu.load_and_run(vec![0xc8, 0xc8]).unwrap();

        assert_eq!(cpu.register_y, 1);
    }
}
