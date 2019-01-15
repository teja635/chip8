extern crate ncurses; 
extern crate sdl2;
use ncurses::*; 
use sdl2::event::{Event};
use sdl2::rect::{Rect};
use sdl2::pixels::{Color};

mod display;
mod cpu;

pub struct CHIP8 {
	disp: display::GFX,
	ctx: sdl2::Sdl,
	video_ctx: sdl2::VideoSubsystem, 
	timer: sdl2::TimerSubsystem, 
	canvas: sdl2::render::Canvas<sdl2::video::Window>,
	processor: cpu::CPU,
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
			})
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
		let byte: u8 = 0b1111_1111;
		self.disp.write_to_gfx(60, 0, byte);
		self.processor.dump_memory(0x200, 0x250);
		
		let mut events = self.ctx.event_pump().unwrap();
		
		'event: loop {
			for event in events.poll_iter() {
				match event {
					Event::Quit{..} => break 'event, 
					Event::KeyDown{timestamp, window_id, keycode, scancode, keymod, repeat} => {
						self.processor.clock_cycle(&mut self.disp);
						self.display();
						self.processor.dump_registers();
					},
					_								=> continue,
				}
			}
		}
	}
}
