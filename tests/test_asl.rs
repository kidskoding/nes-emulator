mod test_asl {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_asl_normal() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0100_0001;

        cpu.load_and_run(vec![0x0a]).unwrap();

        assert_eq!(cpu.register_a, 0b1000_0010);
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
        assert_eq!(cpu.status & 0b0000_0100, 0);
    }

    #[test]
    fn test_asl_zero() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x00;

        cpu.load_and_run(vec![0x0a]).unwrap();

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
        assert_eq!(cpu.status & 0b1000_0000, 0);
        assert_eq!(cpu.status & 0b0000_0100, 0);
    }

    #[test]
    fn test_asl_carry() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x80;

        cpu.load_and_run(vec![0x0a]).unwrap();

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
        assert_eq!(cpu.status & 0b1000_0000, 0);
        assert_eq!(cpu.status & 0b0000_0001, 0b0000_0001);
    }

    #[test]
    fn test_asl_negative() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x7F;

        cpu.load_and_run(vec![0x0a]).unwrap();

        assert_eq!(cpu.register_a, 0xFE);
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
        assert_eq!(cpu.status & 0b0000_0100, 0);
    }

    #[test]
    fn test_asl_multiple_shifts() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b0100_0001;

        cpu.load_and_run(vec![0x0a]).unwrap();
        assert_eq!(cpu.register_a, 0b1000_0010);

        cpu.load_and_run(vec![0x0a]).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0100);

        cpu.load_and_run(vec![0x0a]).unwrap();
        assert_eq!(cpu.register_a, 0b0000_1000);

        cpu.load_and_run(vec![0x0a]).unwrap();
        assert_eq!(cpu.register_a, 0b0001_0000);
    }
}
