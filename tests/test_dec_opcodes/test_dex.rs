mod test_dex {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_dex() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x05;
        cpu.load_and_run(vec![0xCA, 0x00]).unwrap();
        
        assert_eq!(cpu.register_x, 0x04);
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }
    
    #[test]
    fn test_dex_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x01;
        cpu.load_and_run(vec![0xCA, 0x00]).unwrap();
        
        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }
    
    #[test]
    fn test_dex_wraparound() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x00;
        cpu.load_and_run(vec![0xCA, 0x00]).unwrap();
        
        assert_eq!(cpu.register_x, 0xFF);
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
    }
}