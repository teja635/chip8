extern crate sdl2;

use sdl2::event::{Event};
use sdl2::rect::{Rect};
use sdl2::pixels::{Color};
use std::{thread, time};

mod display;
mod cpu;
mod keypad; 
mod assembler;

pub struct CHIP8 {
	disp: display::GFX,
	ctx: sdl2::Sdl,
	video_ctx: sdl2::VideoSubsystem, 
	timer: sdl2::TimerSubsystem, 
	canvas: sdl2::render::Canvas<sdl2::video::Window>,
	processor: cpu::CPU,
	keypad: keypad::Keypad,
}

pub fn init(program: &str) -> Result<CHIP8, &'static str> {
	let ctx = sdl2::init().unwrap();
	let video_ctx = ctx.video().unwrap();
	let timer = ctx.timer().unwrap();
	let window = match video_ctx.window("chip8", 1280, 640).position_centered().opengl().build() {
		Ok(window) => window, 
		Err(err) => panic!("failed to create window: {}", err),
	};

	let canvas = window.into_canvas().present_vsync().build().unwrap();

	Ok( CHIP8 {
				disp: display::init_gfx(),
				ctx: ctx,
				video_ctx: video_ctx,
				timer: timer,
				canvas: canvas,
				processor: cpu::init(program).unwrap(),
				keypad: keypad::init().unwrap(),
			})
}

pub fn assemble(program: &str, output_binary: &str) {
	assembler::assemble(program, output_binary);
}

impl CHIP8 {
	pub fn init_screen(&mut self) {
		self.canvas.clear();
	}

	pub fn display(&mut self) {
		for x in 0..64 {
			for y in 0..32 {
				match self.disp.pixel_at(x, y).unwrap() {
					true => self.canvas.set_draw_color(Color::RGB(255, 255, 255)), 
					false => self.canvas.set_draw_color(Color::RGB(0, 0, 0)),
				}
				self.canvas.fill_rect(Rect::new(20*x as i32, 20*y as i32, 20,  20));
			}
		} self.canvas.present();
	}

	pub fn run(&mut self) {
		let mut events = self.ctx.event_pump().unwrap();
		
		'event: loop {
			for event in events.poll_iter() {
				match event {
					Event::Quit{..} => break 'event, 
					Event::KeyDown{timestamp, window_id, keycode, scancode, keymod, repeat} => {
						self.keypad.keydown(keycode.unwrap());
					},
					Event::KeyUp{timestamp, window_id, keycode, scancode, keymod, repeat} => {
						self.keypad.keyup(keycode.unwrap());
					},
					_								=> continue,
				}
			}
			let changed_disp = self.processor.clock_cycle(&mut self.disp, &mut self.keypad);
			if changed_disp {
				self.display();
			}
			thread::sleep(time::Duration::from_millis(5));
		}
	}
}
