
use speedy2d::{window::{WindowHandler, WindowHelper, VirtualKeyCode, KeyScancode, WindowFullscreenMode, MouseButton, MouseScrollDistance, WindowStartupInfo}, dimen::Vec2, Graphics2D};

use crate::{handler::{MyWindowHandler, Lattice}};


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
				
			}
			Some(VirtualKeyCode::Right) => {
				
			}
			Some(VirtualKeyCode::Up) => {
				
			}
			Some(VirtualKeyCode::Down) => {
				
			}
			
			Some(VirtualKeyCode::Minus) => self.da *= 0.5,
			Some(VirtualKeyCode::Equals) => self.da *= 2.0,
			Some(VirtualKeyCode::Comma) => {
				self.angle -= self.da;
				self.angle = (self.angle * 1000.0) as i32 as f64 * 0.001;
			}
			Some(VirtualKeyCode::Period) => {
				self.angle += self.da;
				self.angle = (self.angle * 1000.0) as i32 as f64 * 0.001;
			}
			Some(VirtualKeyCode::Slash) => self.angle = 0.0,
			
			Some(VirtualKeyCode::F) => self.show_fixed = !self.show_fixed,
			Some(VirtualKeyCode::R) => self.show_rotated = !self.show_rotated,
			Some(VirtualKeyCode::J) => { self.show_atoms = true; self.show_bonds = true }
			Some(VirtualKeyCode::K) => { self.show_atoms = true; self.show_bonds = false }
			Some(VirtualKeyCode::L) => { self.show_atoms = false; self.show_bonds = true }
			
			Some(VirtualKeyCode::S) => self.screenshot = true,
			
			Some(VirtualKeyCode::Z) => self.fixed_lattice = Lattice::PdSe2(self.a, self.b, self.pd_color, self.se_color),
			Some(VirtualKeyCode::X) => self.fixed_lattice = Lattice::TMD(self.a_wse2, self.w_color, self.se_color),
			Some(VirtualKeyCode::C) => self.fixed_lattice = Lattice::TMD(self.a_mose2, self.mo_color, self.se_color),
			Some(VirtualKeyCode::V) => self.fixed_lattice = Lattice::TMD(self.a_ws2, self.w_color, self.s_color),
			Some(VirtualKeyCode::B) => self.fixed_lattice = Lattice::TMD(self.a_mos2, self.mo_color, self.s_color),
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


