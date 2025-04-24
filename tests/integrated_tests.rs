mod integrated_tests {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]).unwrap();

        assert_eq!(cpu.register_x, 0xc1);
    }

    #[test]
    fn test_unknown_opcode() {
        let mut cpu = CPU::new();
        let res = cpu.load_and_run(vec![0x01, 0x04]);

        assert!(res.is_err());
    }
}
