mod test_inx {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_inx() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;

        cpu.load_and_run(vec![0xe8, 0x00]).unwrap();

        assert_eq!(cpu.register_x, 0x02);
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }
    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.load_and_run(vec![0xe8, 0xe8, 0x00]).unwrap();

        assert_eq!(cpu.register_x, 1);
    }
}
