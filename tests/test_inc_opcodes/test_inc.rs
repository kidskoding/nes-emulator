mod test_inc {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_inc_zero_page() {
        let mut cpu = CPU::new();
        
        cpu.mem_write(0x10, 0x05);
        cpu.load(vec![0xE6, 0x10, 0x00]);
        cpu.run().unwrap();
        
        assert_eq!(cpu.mem_read(0x10), 0x06);
        assert_eq!(cpu.status & 0b0000_0010, 0); // Z flag clear
        assert_eq!(cpu.status & 0b1000_0000, 0); // N flag clear
    }
    
    #[test]
    fn test_inc_zero_page_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x05;
        
        cpu.mem_write(0x15, 0x45);
        cpu.load(vec![0xF6, 0x10, 0x00]);
        cpu.run().unwrap();
        
        assert_eq!(cpu.mem_read(0x15), 0x46);
    }
    
    #[test]
    fn test_inc_absolute() {
        let mut cpu = CPU::new();
        
        cpu.mem_write(0x2000, 0x42);
        cpu.load(vec![0xEE, 0x00, 0x20, 0x00]);
        cpu.run().unwrap();
        
        assert_eq!(cpu.mem_read(0x2000), 0x43);
    }
    
    #[test]
    fn test_inc_absolute_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x10;
        
        cpu.mem_write(0x2010, 0x99);
        cpu.load(vec![0xFE, 0x00, 0x20, 0x00]);
        cpu.run().unwrap();
        
        assert_eq!(cpu.mem_read(0x2010), 0x9A);
    }
    
    #[test]
    fn test_inc_zero_flag() {
        let mut cpu = CPU::new();
        
        cpu.mem_write(0x10, 0xFF);
        cpu.load(vec![0xE6, 0x10, 0x00]);
        cpu.run().unwrap();
        
        assert_eq!(cpu.mem_read(0x10), 0x00);
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
    }
    
    #[test]
    fn test_inc_negative_flag() {
        let mut cpu = CPU::new();
        
        cpu.mem_write(0x10, 0x7F);
        cpu.load(vec![0xE6, 0x10, 0x00]);
        cpu.run().unwrap();
        
        assert_eq!(cpu.mem_read(0x10), 0x80);
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
    }
}