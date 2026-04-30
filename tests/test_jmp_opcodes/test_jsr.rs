mod test_jsr {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_jsr_stores_return_address() {
        let mut cpu = CPU::new();
        
        cpu.load(vec![0x20, 0x34, 0x12, 0x00]);
        cpu.mem_write(0x1234, 0x00);
        cpu.run().unwrap();
        
        assert_eq!(cpu.program_counter, 0x1235);
        
        let stored_low_byte = cpu.mem_read(0x01FE);
        let stored_high_byte = cpu.mem_read(0x01FF);
        let stored_addr = ((stored_high_byte as u16) << 8) | (stored_low_byte as u16);
        
        assert_eq!(stored_addr, 0x8002);
    }
    
    #[test]
    fn test_jsr_absolute_addressing_mode() {
        let mut cpu = CPU::new();
        
        cpu.load(vec![0x20, 0xCD, 0xAB, 0x00]);
        cpu.mem_write(0xABCD, 0x00);
        cpu.run().unwrap();
        
        assert_eq!(cpu.program_counter, 0xABCE);
    }

    // test commit lmao
    #[test]
    fn test_jsr_at_page_boundary() {
        let mut cpu = CPU::new();
        
        cpu.mem_write(0x80FD, 0x20);
        cpu.mem_write(0x80FE, 0x34);
        cpu.mem_write(0x80FF, 0x12);
        cpu.mem_write(0x1234, 0x00);
        
        cpu.program_counter = 0x80FD;
        cpu.run().unwrap();
        
        assert_eq!(cpu.program_counter, 0x1235);
        
        let stored_low_byte = cpu.mem_read(0x01FE);
        let stored_high_byte = cpu.mem_read(0x01FF);
        let stored_addr = ((stored_high_byte as u16) << 8) | (stored_low_byte as u16);
        
        assert_eq!(stored_addr, 0x80FF);
    }
}
