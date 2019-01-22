/**
 * Graphics drivers
 *
 */
pub struct GFX {
	pub memory: [u64; 32],
}

pub fn init_gfx() -> GFX {
	GFX { memory: [0; 32] }
}

impl GFX {
	pub fn dump_gfx_mem(&self) {
		for line in self.memory.iter() {
			println!("{:0>64b}", line);
		}
	}

	pub fn write_to_gfx(&mut self, x: usize, y: usize, sprite: u8) {
		let mask: u64 = ((sprite as u64) << (64 - 8)) >> x; 
		self.memory[y] ^= mask;

		if x > (64 - 8) {
			let offset = 8 - (64 - x);
			let mask: u64 = (sprite as u64) << (64 - offset);
			self.memory[y] ^= mask;
		}
	}

	pub fn gfx_memory(&self) -> [u64; 32] {
		self.memory 
	}

	pub fn pixel_at(&self, x: usize, y: usize) -> Result<bool, ()> {
		if x < 64 {
        Ok(self.memory[y] & ((1 as u64) << (64 - 1 - x)) != 0)
    } else {
        Err(())
    }
	}

	pub fn clear_display(&mut self) {
		for idx in 0..32 {
			self.memory[idx] = 0;
		}
	}
}
