mod test_bmi {
	use nes_emulator::cpu::CPU;

	#[test]
	fn test_bmi_branch() {
		let mut cpu = CPU::new();
		cpu.status = 0b1000_0000;

		cpu.load_and_run(vec![0x30, 0x05]).unwrap();
		assert_eq!(cpu.program_counter, 0x8008);
	}

	#[test]
	fn test_bmi_no_branch() {
		let mut cpu = CPU::new();
		cpu.status = 0b0000_0000;

		cpu.load_and_run(vec![0x30, 0x05]).unwrap();
		assert_eq!(cpu.program_counter, 0x8003);
	}
}