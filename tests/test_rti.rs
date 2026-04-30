mod test_rti {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_rti() {
        let mut cpu = CPU::new();
        // Push target PC and status
        cpu.stack_push_u16(0x1234);
        cpu.stack_push(0b1100_0001); // Status with Carry set
        
        cpu.load_and_run(vec![0x40, 0x00]).unwrap();
        
        assert_eq!(cpu.program_counter, 0x1235); // 0x1234 + 1 from BRK/run loop logic
        assert_eq!(cpu.status & 1, 1);
    }
}
