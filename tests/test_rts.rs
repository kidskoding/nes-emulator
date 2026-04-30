mod test_rts {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_rts() {
        let mut cpu = CPU::new();
        // Simulate JSR to 0x1234 from 0x8000
        // JSR at 0x8000 pushes 0x8000 + 2 - 1 = 0x8001
        // PC is then set to 0x1234
        // RTS at 0x1234 pulls 0x8001 and sets PC to 0x8001 + 1 = 0x8002
        
        cpu.load(vec![0x20, 0x34, 0x12, 0x00]); // JSR 0x1234, then BRK
        cpu.mem_write(0x1234, 0x60); // RTS at 0x1234
        
        cpu.run().unwrap();
        
        assert_eq!(cpu.program_counter, 0x8004); // 0x8003 is BRK, +1 from run loop
    }
}
