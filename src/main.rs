use chip8;
use chip8::CHIP8;
use std::process;

fn main() {
	let mut chip8 = chip8::init().unwrap_or_else(|err| {
		println!("[-] Problem generating chip8: {}", err);
		process::exit(1);
	});

	chip8.init_screen();
	chip8.clock_cycle();
}
