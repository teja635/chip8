extern crate ncurses; 

use ncurses::*; 

mod display;

pub struct CHIP8 {
	disp: display::GFX,
}

pub fn init() -> Result<CHIP8, &'static str> {
	Ok( CHIP8 {disp: display::init_gfx() } )
}

impl CHIP8 {
	pub fn init_screen(&self) {
		let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
	}

	pub fn display(&self) {
		for line in (0..32).step_by(2) {
			for pixel in 0..64 {
				let upper_pixel = self.disp.pixel_at(pixel, line).unwrap();
				let lower_pixel = self.disp.pixel_at(pixel, line + 1).unwrap();

				match (upper_pixel, lower_pixel) {
					(true, true) => printw("\u{2588}"),
					(false, true) => printw("\u{2584}"),
					(true, false) => printw("\u{2580}"),
					(false, false) => printw(" "),
				};
			}
			printw("\n");
		}

		refresh();
	}

	pub fn clock_cycle(&mut self) {
		clear();
		let byte: u8 = 0b1111_1111;
		self.disp.write_to_gfx(0, 0, byte);
		self.display();
		getch();
		endwin();
	}
}
