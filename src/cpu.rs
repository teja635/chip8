extern crate rand; 

use std::io::{Read};
use std::fs::File;

use crate::display;
use crate::keypad;

use rand::Rng;

pub struct CPU {
	V: [u8; 16],
	I: u16,
	pc: u16,
	memory: [u8; 4096],
	stack: [u16; 16],
	sp: u16,
	dt: u8,
	st: u8,
}

pub fn init(program: &str) -> Result<CPU, &'static  str> {
	let mut memory: [u8; 4096] = [0; 4096];
	let mut fd = File::open(program).unwrap();
	fd.read(&mut memory[0x200..]);

	let letters = [
		0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
		0x20, 0x60, 0x20, 0x20, 0x70, // 1
		0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
		0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
		0x90, 0x90, 0xF0, 0x10, 0x10, // 4
		0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
		0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
		0xF0, 0x10, 0x20, 0x40, 0x40, // 7
		0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
		0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
		0xF0, 0x90, 0xF0, 0x90, 0x90, // A
		0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
		0xF0, 0x80, 0x80, 0x80, 0xF0, // C
		0xE0, 0x90, 0x90, 0x90, 0xE0, // D
		0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
		0xF0, 0x80, 0xF0, 0x80, 0x80  // F
	];
	
	for idx in 0..80 {
		memory[idx] = letters[idx];
	}

	Ok( CPU {
		V: [0; 16],
		I: 0,
		pc: 0x200, 
		memory: memory,
		stack: [0; 16],
		sp: 0,
		dt: 0,
		st: 0,
	})
}

fn get_reg_x(opcode: u16) -> usize {
	((opcode & 0x0F00) >> 8) as usize
}

fn get_reg_y(opcode: u16) -> usize {
	((opcode & 0x00F0) >> 4) as usize
}

fn get_n(opcode: u16) -> u8 {
	(opcode & 0x000F) as u8 
}

fn get_kk(opcode: u16) -> u8 {
	(opcode & 0x00FF) as u8
}

impl CPU {
	pub fn dump_memory(&self, start_idx: usize, end_idx: usize) {
		for idx in start_idx..end_idx {
			println!("[{:x}] {:x}", idx, self.memory[idx]);
		}
	}

	fn stack_pop(&mut self) -> Result<u16, &'static str>{
		if self.sp == 0 {
			Err("[-] Stack is empty")
		}
		else {
			let res = self.stack[(self.sp - 1) as usize];
			self.sp -= 1; 
			Ok(res)
		}
	}

	fn push_stack(&mut self, elem: u16) -> Result<(), &'static str> {
		if self.sp == 16 {
			Err("[-] Stack is full")
		}
		else {
			self.stack[self.sp as usize] = elem;
			self.sp += 1;
			Ok(())
		}
	}
	
	pub fn dump_registers(&self) {
		for (i, item) in self.V.iter().enumerate() {
			println!("[V{}] = {}", i, item);
		}
	}

	pub fn clock_cycle(&mut self, mut disp: &mut display::GFX, mut keys: &mut keypad::Keypad) -> bool{
		let opcode = ((self.memory[self.pc as usize] as u16) << 8) | 
			(self.memory[self.pc as usize + 1] as u16);
		println!("[+] Performing opcode {:x}", opcode);
		match opcode & 0xF000 {
			0x0000 => {
				match opcode & 0x000F {
					0x0000 => {
						match opcode & 0x00E0 {
							0x00E0 	=> {
								disp.clear_display();
								self.pc += 2;
							},
							_ 			=> panic!("[-] Invalid Opcode {:x}", opcode),
						}
					},
					0x000E => {
						if opcode == 0x00EE {
							self.pc = self.stack_pop().unwrap();
						}
					},
					_ => panic!("[-] Invalid Opcode {:x}", opcode),
				}
			},
			0x1000 => {
				self.pc = opcode & 0x0FFF;
			},
			0x2000 => {
				self.push_stack(self.pc).unwrap();
				self.pc = opcode & 0x0FFF;
			},
			0x3000 => {
				let x = get_reg_x(opcode);
				let kk = get_kk(opcode);
				
				self.pc = self.pc + (if self.V[x] == kk { 4 } else { 2 });
			},
			0x4000 => {
				let x = get_reg_x(opcode);
				let kk = get_kk(opcode);
				
				self.pc = self.pc + (if self.V[x] != kk { 4 } else { 2 });
			},
			0x5000 => {
				let x = get_reg_x(opcode);
				let y = get_reg_y(opcode);
				
				self.pc = self.pc + (if self.V[x] == self.V[y] { 4 } else { 2 });
			},
			0x6000 => {
				let x = get_reg_x(opcode);

				self.V[x] = (opcode & 0x00FF) as u8;
				self.pc += 2; 
			},
			0x7000 => {
				let x = get_reg_x(opcode);

				self.V[x] += (opcode & 0x00FF) as u8;
				self.pc += 2; 
			},
			0x8000 => {
				let x = get_reg_x(opcode);
				let y = get_reg_y(opcode);
				
				match opcode & 0x000F {
					0x0000 => self.V[x] = self.V[y],
					0x0001 => self.V[x] = self.V[x] | self.V[y],
					0x0002 => self.V[x] = self.V[x] & self.V[y],
					0x0003 => self.V[x] = self.V[x] ^ self.V[y],
					0x0004 => self.V[x] = self.V[x] + self.V[y],
					0x0005 => self.V[x] = self.V[x] - self.V[y],
					_ => panic!("[-] function for {} not implemented yet", opcode),
				}

				self.pc += 2; 
			},
			0x9000 => {
				let x = get_reg_x(opcode);
				let y = get_reg_y(opcode);
				
				self.pc = self.pc + (if self.V[x] == self.V[y] { 4 } else { 2 });
			},
			0xA000 => {
				self.I = opcode & 0x0FFF; 
				self.pc += 2; 
			},
			0xB000 => {
				self.pc = (self.V[0] as u16) + (0x0FFF & opcode)
			},
			0xC000 => {
				let x = get_reg_x(opcode);
				let mut rng = rand::thread_rng();

				let rand_byte: u8 = rng.gen();
				self.V[x] = rand_byte & get_kk(opcode);
				self.pc += 2; 
			},
			0xD000 => {
				let n = get_n(opcode);
				let x = get_reg_x(opcode);
				let y = get_reg_y(opcode);
				

				for i in 0..n {
					disp.write_to_gfx(self.V[x] as usize, (self.V[y] + i) as usize, self.memory[(self.I + (i as u16)) as usize]);
				}
				self.pc += 2; 
				return true
			},
			0xE000 => {
				match opcode & 0x00FF {
					0x009E => {
						let x = get_reg_x(opcode);
						
						self.pc += if keys.is_pressed(self.V[x]).unwrap() { 4 } else { 2 };
					},
					0x00A1 => {
						let x = get_reg_x(opcode);
						
						self.pc += if !keys.is_pressed(self.V[x]).unwrap() { 4 } else { 2 };
					},
					_ => {},
				}
			},
			0xF000 => {
				match opcode & 0x00FF {
					0x000F => {
						let x = get_reg_x(opcode);
						self.V[x] = self.dt;
					},
					0x000A => {
						self.pc += 2; 
					},
					0x0015 => {
						let x = get_reg_x(opcode);
						self.dt = self.V[x];
					},
					0x0018 => {
						panic!("[-] Sound timer is not yet set.");
					},
					0x001E => {
						let x = get_reg_x(opcode);
						self.I = self.I + self.V[x] as u16;
						self.pc += 2;
					},
					0x0029 => {
						let x = get_reg_x(opcode);
						self.I = (self.V[x] as u16) * 5; 
						self.pc += 2;
					}, 
					0x0033 => {
						let x = get_reg_x(opcode);
						self.memory[self.I as usize] = (self.V[x]/100) % 10;
						self.memory[(self.I + 1) as usize] = (self.V[x]/10) % 10;
						self.memory[(self.I + 2) as usize] = self.V[x] % 10;
						self.pc += 2; 
					}, 
					0x0055 => {
						let x = get_reg_x(opcode);
						for reg in 0..x {
							self.memory[(self.I as usize) + reg] = self.V[reg];
						}
						self.pc += 2; 
					},
					0x0065 => {
						let x = get_reg_x(opcode);
						for reg in 0..x {
							self.V[reg] = self.memory[(self.I as usize) + reg];
						}
						self.pc += 2; 
					},
					_ => panic!("[-] Invalid instruction {:x}", opcode),
				}
			},
			_ => panic!("[-] Invalid Opcode {:x}", opcode),
		}
		return false
	}
}
