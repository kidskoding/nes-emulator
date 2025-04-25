mod test_cmp {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_cmp_equal() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        cpu.load_and_run(vec![0xC9, 0x05, 0x00]).unwrap();
        
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
        assert_eq!(cpu.status & 0b0000_0001, 0b0000_0001);
        assert_eq!(cpu.status & 0b1000_0000, 0);          
    }

    #[test]
    fn test_cmp_greater_than() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x10; 
        cpu.load_and_run(vec![0xC9, 0x05, 0x00]).unwrap();
        
        assert_eq!(cpu.status & 0b0000_0010, 0);            
        assert_eq!(cpu.status & 0b0000_0001, 0b0000_0001);  
        assert_eq!(cpu.status & 0b1000_0000, 0);            
    }

    #[test]
    fn test_cmp_less_than() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05; 
        cpu.load_and_run(vec![0xC9, 0x0A, 0x00]).unwrap();
        
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b0000_0001, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
    }

    #[test]
    fn test_cmp_negative_result() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x02;
        cpu.load_and_run(vec![0xC9, 0x81, 0x00]).unwrap();
        
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b0000_0001, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
    }

    #[test]
    fn test_cmp_zero_page() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x42;
        cpu.mem_write(0x10, 0x42);
        cpu.load_and_run(vec![0xC5, 0x10, 0x00]).unwrap();
        
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010); 
        assert_eq!(cpu.status & 0b0000_0001, 0b0000_0001); 
        assert_eq!(cpu.status & 0b1000_0000, 0);           
    }

    #[test]
    fn test_cmp_zero_page_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x42;
        cpu.register_x = 0x05;
        cpu.mem_write(0x15, 0x42);
        cpu.load_and_run(vec![0xD5, 0x10, 0x00]).unwrap();
        
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
        assert_eq!(cpu.status & 0b0000_0001, 0b0000_0001); 
        assert_eq!(cpu.status & 0b1000_0000, 0);           
    }
}