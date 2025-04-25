pub mod test_dey {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_dey() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x10;
        cpu.load_and_run(vec![0x88, 0x00]).unwrap();
        
        assert_eq!(cpu.register_y, 0x0F);
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }
    
    #[test]
    fn test_dey_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x01;
        cpu.load_and_run(vec![0x88, 0x00]).unwrap();
        
        assert_eq!(cpu.register_y, 0x00);
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }
    
    #[test]
    fn test_dey_wraparound() {
        let mut cpu = CPU::new();
        
        cpu.register_y = 0x00;
        cpu.load_and_run(vec![0x88, 0x00]).unwrap();
        
        assert_eq!(cpu.register_y, 0xFF);
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
    }
}