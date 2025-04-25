mod test_dec {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_dec_zeropage() {
        let mut cpu = CPU::new();
        
        cpu.mem_write(0x10, 0x05);
        cpu.load_and_run(vec![0xC6, 0x10, 0x00]).unwrap();
        
        assert_eq!(cpu.mem_read(0x10), 0x04);
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }
    
    #[test]
    fn test_dec_zero_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x01);
        cpu.load_and_run(vec![0xC6, 0x10, 0x00]).unwrap();
        
        assert_eq!(cpu.mem_read(0x10), 0x00);
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }
    
    #[test]
    fn test_dec_negative_flag() {
        let mut cpu = CPU::new();
        
        cpu.mem_write(0x10, 0x00);
        cpu.load_and_run(vec![0xC6, 0x10, 0x00]).unwrap();
        
        assert_eq!(cpu.mem_read(0x10), 0xFF);
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
    }
    
    #[test]
    fn test_dec_absolute() {
        let mut cpu = CPU::new();
        
        cpu.mem_write(0x2000, 0x42);
        cpu.load_and_run(vec![0xCE, 0x00, 0x20, 0x00]).unwrap();
        assert_eq!(cpu.mem_read(0x2000), 0x41);
    }
}