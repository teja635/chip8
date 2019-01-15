use chip8;
use chip8::CHIP8;
use std::process;
use std::env; 

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut chip8: CHIP8 = chip8::init(&args[1]).unwrap_or_else(|err| {
		println!("[-] Problem generating chip8: {}", err);
		process::exit(1);
	});

	chip8.init_screen();
	chip8.run();
}
