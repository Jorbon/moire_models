
use std::{time::{Instant, Duration}, f64::consts::PI};
use speedy2d::{dimen::{Vector2}, font::Font, color::Color};

pub enum Lattice {
	PdSe2(f64, f64, Color, Color),
	TMD(f64, Color, Color)
}

pub struct MyWindowHandler {
	pub size: Vector2<u32>,
	pub fullscreen: bool,
	pub mouse_x: f32,
	pub mouse_y: f32,
	pub mouse_left: bool,
	pub frame_time_avg: f64,
	pub vx: f64,
	pub vy: f64,
	pub vz: f64,
	pub u: f64,
	pub v: f64,
	pub w: f64,
	pub fov: f64,
	pub font: Font,
	pub show_atoms: bool,
	pub show_bonds: bool,
	pub show_fixed: bool,
	pub show_rotated: bool,
	pub show_overlay: bool,
	pub fixed_lattice: Lattice,
	pub angle: f64,
	pub da: f64,
	pub frame_timer: Instant,
	pub previous_frame_timestamp: Duration,
	pub screenshot: bool,
	pub screenshot_dot: bool,
	pub pd_color: Color,
	pub se_color: Color,
	pub s_color: Color,
	pub w_color: Color,
	pub mo_color: Color,
	pub bond_color: Color,
	pub r: f64,
	pub bond_r: f64,
	pub a: f64,
	pub b: f64,
	pub a_wse2: f64,
	pub a_mose2: f64,
	pub a_ws2: f64,
	pub a_mos2: f64,
	pub bilayer: bool,
	pub static_bilayer: bool,
}

impl MyWindowHandler {
	pub fn new(size: Vector2<u32>) -> Self {
		let pd_color = Color::from_rgb(0.1, 0.7, 0.7);
		let se_color = Color::from_rgb(0.1, 0.8, 0.1);
		let a = 5.755;
		let b = 5.901;
		
		Self {
			size,
			fullscreen: false,
			mouse_x: size.x as f32 * 0.5,
			mouse_y: size.y as f32 * 0.5,
			mouse_left: false,
			frame_time_avg: 0.0,
			frame_timer: Instant::now(),
			previous_frame_timestamp: Duration::ZERO,
			vx: 0.0,
			vy: 0.0,
			vz: 0.0,
			u: 0.0,
			v: 0.0,
			w: 0.0,
			fov: 100.0,
			font: Font::new(include_bytes!("../assets/fonts/NotoSans-Regular.ttf")).unwrap(),
			show_atoms: true,
			show_bonds: true,
			show_fixed: true,
			show_rotated: true,
			show_overlay: true,
			fixed_lattice: Lattice::PdSe2(a, b, pd_color, se_color),
			angle: 0.0,
			da: 0.1,
			screenshot: false,
			screenshot_dot: false,
			pd_color,
			se_color,
			s_color: Color::from_rgb(0.7, 0.7, 0.1),
			w_color: Color::from_rgb(0.1, 0.5, 0.9),
			mo_color: Color::from_rgb(0.6, 0.1, 0.9),
			bond_color: Color::from_rgb(0.7, 0.7, 0.7),
			r: 0.5,
			bond_r: 0.15,
			a,
			b,
			a_wse2: 3.32,
			a_mose2: 3.322,
			a_ws2: 3.184,
			a_mos2: 3.193,
			bilayer: false,
			static_bilayer: true,
		}
	}
	
	pub fn get_output_scale(&self) -> f64 {
		0.5 * self.size.x as f64 / self.fov
	}
	
	pub fn get_output_transform(&self, rotation: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
		let output_scale = self.get_output_scale();
		let cos = f64::cos(rotation * PI/180.0);
		let sin = f64::sin(rotation * PI/180.0);
		(
			cos * output_scale,
			sin * output_scale,
			-sin * output_scale,
			cos * output_scale,
			0.5 * self.size.x as f64 - self.vx * output_scale,
			0.5 * self.size.y as f64 - self.vy * output_scale,
			output_scale,
		)
	}
	
	pub fn _output(&self, x: f64, y: f64) -> (f32, f32) {
		let cosu = self.u.cos();
		let sinu = self.u.sin();
		let out_x = x*cosu + y*sinu;
		let out_y = y*cosu - x*sinu;
		(
			(self.size.x as f64 * 0.5 * (1.0 + (out_x - self.vx) / self.fov)) as f32,
			0.5 * (self.size.y as f64 + self.size.x as f64 * (out_y - self.vy) / self.fov) as f32
		)
	}
	
	pub fn input(&self, x: f32, y: f32) -> (f64, f64) {
		(
			self.fov * (x as f64 * 2.0 / self.size.x as f64 - 1.0) + self.vx,
			self.fov * (y as f64 * 2.0 - self.size.y as f64) / self.size.x as f64 + self.vy
		)
	}
	
	pub fn input_rotated(&self, x: f32, y: f32) -> (f64, f64) {
		let (in_x, in_y) = self.input(x, y);
		let sin = f64::sin(self.angle * PI/180.0);
		let cos = f64::cos(self.angle * PI/180.0);
		(
			in_x*cos + in_y*sin,
			in_y*cos - in_x*sin,
		)
	}
}


