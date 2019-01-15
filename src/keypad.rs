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
			Keycode::Num1 => { self.keys[0x1] = true; },
			Keycode::Num2 => { self.keys[0x2] = true; },
			Keycode::Num3 => { self.keys[0x3] = true; },
			Keycode::Num4 => { self.keys[0xC] = true; },
			Keycode::Q => { self.keys[0x4] = true; },
			Keycode::W => { self.keys[0x5] = true; },
			Keycode::E => { self.keys[0x6] = true; },
			Keycode::R => { self.keys[0xD] = true; },
			Keycode::A => { self.keys[0x7] = true; },
			Keycode::S => { self.keys[0x8] = true; },
			Keycode::D => { self.keys[0x9] = true; },
			Keycode::F => { self.keys[0xE] = true; },
			Keycode::Z => { self.keys[0xA] = true; },
			Keycode::X => { self.keys[0x0] = true; },
			Keycode::C => { self.keys[0xB] = true; },
			Keycode::V => { self.keys[0xF] = true; },
			_					 => {},
		};
	}

	pub fn keyup(&mut self, keycode: Keycode) {
		match keycode {
			Keycode::Num1 => { self.keys[0x1] = false; },
			Keycode::Num2 => { self.keys[0x2] = false; },
			Keycode::Num3 => { self.keys[0x3] = false; },
			Keycode::Num4 => { self.keys[0xC] = false; },
			Keycode::Q => { self.keys[0x4] = false; },
			Keycode::W => { self.keys[0x5] = false; },
			Keycode::E => { self.keys[0x6] = false; },
			Keycode::R => { self.keys[0xD] = false; },
			Keycode::A => { self.keys[0x7] = false; },
			Keycode::S => { self.keys[0x8] = false; },
			Keycode::D => { self.keys[0x9] = false; },
			Keycode::F => { self.keys[0xE] = false; },
			Keycode::Z => { self.keys[0xA] = false; },
			Keycode::X => { self.keys[0x0] = false; },
			Keycode::C => { self.keys[0xB] = false; },
			Keycode::V => { self.keys[0xF] = false; },
			_					 => {},
		};
	}

	pub fn is_pressed(&mut self, key: u8) -> Result<bool, &'static str> {
		if key <= 0x0F {
			Ok(self.keys[key as usize])
		} else {
			Err("[-] Invalid key")
		}
	}
}
