extern crate ncurses; 

use ncurses::*; 

mod display;

struct CHIP8 {
	disp: display::Display,
}

pub fn init() -> Result<Board, &static str> {
	Ok( CHIP8 {disp: display::init_display() } )
}

impl CHIP8 {
	pub fn test(&self) {
		let byte: u8 = 0b1111_1111;
		disp.write_to_display(63, 5, byte);
		disp.dump_display_memory();
	}
}
