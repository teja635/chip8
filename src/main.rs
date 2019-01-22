use chip8;
use chip8::CHIP8;
use std::process;
use std::env; 

fn main() {
	let argv: Vec<String> = env::args().collect();
	
	/* Parsing arguements */
	// FIXME: Invalid file name 
	match argv[1].as_ref() {
		"assemble" => {
			chip8::assemble(&argv[2], &argv[3]);
		},
		"console" => {
			let mut chip8: CHIP8 = chip8::init(&argv[2]).unwrap_or_else(|err| {
				println!("[-] Problem generating chip8: {}", err);
				process::exit(1);
			});

			chip8.init_screen();
			chip8.run();
		},
		_ => {
			eprintln!("[-] Invalid args");
			eprintln!("[-] Usage: cargo run [assemble,console] file");
			eprintln!("[-] Process exitting");
			process::exit(1);
		},
	}
	process::exit(1);
}
