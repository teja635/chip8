extern crate sdl2; 

use sdl2::keyboard::Keycode; 

pub struct Keypad {
	keys: [bool; 16],
}

pub fn init() -> Result<Keypad, &'static str> {
	Ok( Keypad {
		keys: [false; 16],
	})
}

impl Keypad {
	pub fn keydown(&mut self, keycode: Keycode) {
		match keycode {
			Keycode::Num1 => { self.keys[0] = true; },
			Keycode::Num2 => { self.keys[1] = true; },
			Keycode::Num3 => { self.keys[2] = true; },
			Keycode::Num4 => { self.keys[3] = true; },
			Keycode::Q => { self.keys[4] = true; },
			Keycode::W => { self.keys[5] = true; },
			Keycode::E => { self.keys[6] = true; },
			Keycode::R => { self.keys[7] = true; },
			Keycode::A => { self.keys[8] = true; },
			Keycode::S => { self.keys[9] = true; },
			Keycode::D => { self.keys[10] = true; },
			Keycode::F => { self.keys[11] = true; },
			Keycode::Z => { self.keys[12] = true; },
			Keycode::X => { self.keys[13] = true; },
			Keycode::C => { self.keys[14] = true; },
			Keycode::V => { self.keys[15] = true; },
			_					 => {},
		};
	}

	pub fn keyup(&mut self, keycode: Keycode) {
		match keycode {
			Keycode::Num1 => { self.keys[0] = false; },
			Keycode::Num2 => { self.keys[1] = false; },
			Keycode::Num3 => { self.keys[2] = false; },
			Keycode::Num4 => { self.keys[3] = false; },
			Keycode::Q => { self.keys[4] = false; },
			Keycode::W => { self.keys[5] = false; },
			Keycode::E => { self.keys[6] = false; },
			Keycode::R => { self.keys[7] = false; },
			Keycode::A => { self.keys[8] = false; },
			Keycode::S => { self.keys[9] = false; },
			Keycode::D => { self.keys[10] = false; },
			Keycode::F => { self.keys[11] = false; },
			Keycode::Z => { self.keys[12] = false; },
			Keycode::X => { self.keys[13] = false; },
			Keycode::C => { self.keys[14] = false; },
			Keycode::V => { self.keys[15] = false; },
			_					 => {},
		};
	}
}
