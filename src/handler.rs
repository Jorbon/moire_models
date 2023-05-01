
use std::{time::{Instant, Duration}, rc::Rc, f64::consts::PI};
use speedy2d::{dimen::{Vector2}, font::{Font, FormattedTextBlock}};

use crate::structs::Layer;

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
	pub font1: Option<Font>,
	pub str1: String,
	pub text1: Option<Rc<FormattedTextBlock>>,
	pub text2: Option<Rc<FormattedTextBlock>>,
	pub layers: Vec<Layer>,
	pub da: f64,
	pub frame_timer: Instant,
	pub previous_frame_timestamp: Duration,
	pub screenshot: bool,
}

impl MyWindowHandler {
	pub fn new(size: Vector2<u32>) -> Self {
		let layers = Vec::new();
		
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
			fov: 10.0,
			font1: None,
			str1: String::new(),
			text1: None,
			text2: None,
			layers,
			da: 0.1,
			screenshot: false,
		}
	}
	
	pub fn get_output_transform(&self, rotation: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
		let cosu = f64::cos((self.u - rotation) * PI/180.0);
		let sinu = f64::sin((self.u - rotation) * PI/180.0);
		let cosv = f64::cos(self.v * PI/180.0);
		let sinv = f64::sin(self.v * PI/180.0);
		
		let r = (
			cosu,
			-sinu * cosv,
			sinu,
			cosu * cosv,
			0.0,
			sinv,
		);
		
		(
			r.0 * 0.5 * self.size.x as f64 / self.fov,
			r.1 * 0.5 * self.size.x as f64 / self.fov,
			r.2 * 0.5 * self.size.x as f64 / self.fov,
			r.3 * 0.5 * self.size.x as f64 / self.fov,
			r.4 * 0.5 * self.size.x as f64 / self.fov,
			r.5 * 0.5 * self.size.x as f64 / self.fov,
			0.5 * self.size.x as f64 * (1.0 - self.vx / self.fov),
			0.5 * (self.size.y as f64 - self.size.x as f64 * self.vy / self.fov),
		)
	}
	
	pub fn _output(&self, x: f64, y: f64, z: f64) -> (f32, f32) {
		let cosu = self.u.cos();
		let sinu = self.u.sin();
		let x2 = x*cosu + y*sinu;
		let y2 = y*cosu - x*sinu;
		let cosv = self.v.cos();
		let sinv = self.v.sin();
		let out_x = x2;
		let out_y = -y2*cosv + z*sinv;
		(
			(self.size.x as f64 * 0.5 * (1.0 + (out_x - self.vx) / self.fov)) as f32,
			0.5 * (self.size.y as f64 + self.size.x as f64 * (out_y - self.vy) / self.fov) as f32
		)
	}
	
	pub fn input(&self, x: f32, y: f32) -> (f64, f64, f64) {
		let in_x = x as f64;
		let in_y = y as f64;
		
		(
			self.fov * (in_x * 2.0 / self.size.x as f64 - 1.0) + self.vx,
			-(self.fov * (in_y * 2.0 - self.size.y as f64) / self.size.x as f64 + self.vy),
			self.fov * 0.0
		)
	}
}


