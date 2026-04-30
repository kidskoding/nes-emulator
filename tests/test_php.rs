mod test_php {
    use nes_emulator::cpu::CPU;

    #[test]
    fn test_php() {
        let mut cpu = CPU::new();
        cpu.status = 0b1100_1111;
        cpu.load_and_run(vec![0x08, 0x00]).unwrap();
        assert_eq!(cpu.stack_pointer, 0xFE);
        // Bit 4 and 5 should be set when pushed to stack
        assert_eq!(cpu.mem_read(0x01FF), 0b1111_1111);
    }
}
