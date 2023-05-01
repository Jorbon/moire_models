
extern crate speedy2d;
extern crate png;

use speedy2d::dimen::{Vector2};
use speedy2d::window::{WindowCreationOptions, WindowSize, WindowPosition};
use speedy2d::{Window};

mod structs;
mod util;
mod handler;
mod start;
mod draw;
mod events;

use crate::handler::MyWindowHandler;


fn main() {
	let title = "PdSe2";
	let size = Vector2::<u32>{x: 1440, y: 810};
	let vsync = true;
	
	let options = WindowCreationOptions::new_windowed(WindowSize::ScaledPixels(Vector2 {x: size.x as f32, y: size.y as f32}), Some(WindowPosition::Center)).with_vsync(vsync);
	let window = Window::new_with_options(title, options).unwrap();
	
	let w = MyWindowHandler::new(size);
	
	window.run_loop(w)
}







