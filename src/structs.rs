

extern crate speedy2d;
use speedy2d::color::Color;

#[derive(Clone)]
pub struct Atom {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub r: f64,
	pub color: Color
}

#[derive(Clone)]
pub struct Bond {
	pub index1: usize,
	pub index2: usize,
	pub color: Color
}

#[derive(Clone)]
pub struct Layer {
	pub atoms: Vec<Atom>,
	pub bonds: Vec<Bond>,
	pub angle: f64,
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub name: String,
	pub show: bool
}