
use speedy2d::{window::{WindowHandler, WindowHelper, VirtualKeyCode, KeyScancode, WindowFullscreenMode, MouseButton, MouseScrollDistance, WindowStartupInfo}, dimen::Vec2, Graphics2D};

use crate::{handler::MyWindowHandler};


impl WindowHandler for MyWindowHandler {
	fn on_start(&mut self, helper: &mut WindowHelper<()>, info: WindowStartupInfo) {
		crate::start::start(self, helper, info);
	}
	
	fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
		crate::draw::draw(self, helper, graphics);
	}
	
	fn on_resize(&mut self, _helper: &mut WindowHelper<()>, size_pixels: speedy2d::dimen::UVec2) {
		self.size = size_pixels;
	}
	
	fn on_key_down(&mut self, helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, _scancode: KeyScancode) {
		match virtual_key_code {
			Some(VirtualKeyCode::F11) => {
				self.fullscreen = !self.fullscreen;
				helper.set_fullscreen_mode(match self.fullscreen {
					true => WindowFullscreenMode::FullscreenBorderless,
					false => WindowFullscreenMode::Windowed
				});
			}
			Some(VirtualKeyCode::Escape) => {
				self.fullscreen = false;
				helper.set_fullscreen_mode(WindowFullscreenMode::Windowed);
			}
			
			Some(VirtualKeyCode::Left) => {
				self.u += 1.0;
			}
			Some(VirtualKeyCode::Right) => {
				self.u -= 1.0;
			}
			Some(VirtualKeyCode::Up) => {
				self.v += 1.0;
			}
			Some(VirtualKeyCode::Down) => {
				self.v -= 1.0;
			}
			
			Some(VirtualKeyCode::Minus) => {
				self.da *= 0.5;
				helper.set_title(format!("PdSe2: {} degrees (da: {})", self.layers[1].angle, self.da));
			}
			Some(VirtualKeyCode::Equals) => {
				self.da *= 2.0;
				helper.set_title(format!("PdSe2: {} degrees (da: {})", self.layers[1].angle, self.da));
			}
			Some(VirtualKeyCode::Comma) => {
				for i in 1..self.layers.len() {
					self.layers[i].angle -= self.da;
					self.layers[i].angle = (self.layers[i].angle * 1000.0) as i32 as f64 * 0.001;
				}
				helper.set_title(format!("PdSe2: {} degrees (da: {})", self.layers[1].angle, self.da));
			}
			Some(VirtualKeyCode::Period) => {
				for i in 1..self.layers.len() {
					self.layers[i].angle += self.da;
					self.layers[i].angle = (self.layers[i].angle * 1000.0) as i32 as f64 * 0.001;
				}
				helper.set_title(format!("PdSe2: {} degrees (da: {})", self.layers[1].angle, self.da));
			}
			Some(VirtualKeyCode::Slash) => {
				for i in 1..self.layers.len() {
					self.layers[i].angle = 0.0;
				}
			}
			
			Some(VirtualKeyCode::Z) => { if self.layers.len() > 0 { self.layers[0].show = !self.layers[0].show } }
			Some(VirtualKeyCode::X) => { if self.layers.len() > 1 { self.layers[1].show = !self.layers[1].show } }
			Some(VirtualKeyCode::C) => { if self.layers.len() > 2 { self.layers[2].show = !self.layers[2].show } }
			Some(VirtualKeyCode::V) => { if self.layers.len() > 3 { self.layers[3].show = !self.layers[3].show } }
			Some(VirtualKeyCode::B) => { if self.layers.len() > 4 { self.layers[4].show = !self.layers[4].show } }
			Some(VirtualKeyCode::N) => { if self.layers.len() > 5 { self.layers[5].show = !self.layers[5].show } }
			Some(VirtualKeyCode::M) => { if self.layers.len() > 6 { self.layers[6].show = !self.layers[6].show } }
			_ => ()
		}
	}
	
	fn on_mouse_move(&mut self, _helper: &mut WindowHelper<()>, position: Vec2) {
		let dx = (position.x - self.mouse_x) as f64;
		let dy = (position.y - self.mouse_y) as f64;
		self.mouse_x = position.x;
		self.mouse_y = position.y;
		
		if self.mouse_left {
			self.vx -= dx * 2.0 * self.fov / self.size.x as f64;
			self.vy -= dy * 2.0 * self.fov / self.size.x as f64;
		}
	}
	
	fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<()>, button: MouseButton) {
		if let MouseButton::Left = button {
			self.mouse_left = true;
		}
	}
	
	fn on_mouse_button_up(&mut self, _helper: &mut WindowHelper<()>, button: MouseButton) {
		if let MouseButton::Left = button {
			self.mouse_left = false;
		}
	}
	
	fn on_mouse_wheel_scroll(&mut self, _helper: &mut WindowHelper<()>, distance: MouseScrollDistance) {
		let scroll_distance = match distance {
			MouseScrollDistance::Lines { x:_, y, z:_ } => y,
			MouseScrollDistance::Pixels { x:_, y, z:_ } => y,
			MouseScrollDistance::Pages { x:_, y, z:_ } => y
		} as f64;
		
		let dzoom = self.fov * scroll_distance * -0.05;
		self.fov += dzoom;
		self.vx -= dzoom * (self.mouse_x as f64 / self.size.x as f64 * 2.0 - 1.0);
		self.vy -= dzoom * (self.mouse_y as f64 / self.size.x as f64 * 2.0 - self.size.y as f64 / self.size.x as f64);
	}
}


