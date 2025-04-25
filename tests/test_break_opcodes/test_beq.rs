mod test_beq {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_beq_no_branch() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0000;
        
        cpu.load_and_run(vec![0xF0, 0x03, 0xA9, 0x0A]).unwrap();
        assert_eq!(cpu.program_counter, 0x8005);
    }

    #[test]
    fn test_beq_branch() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0010;
        
        cpu.load_and_run(vec![0xF0, 0x03, 0xA9, 0x0A]).unwrap();
        assert_eq!(cpu.program_counter, 0x8006);
    }
}