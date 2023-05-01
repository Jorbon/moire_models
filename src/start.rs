
use speedy2d::{window::{WindowHelper, WindowStartupInfo}, font::Font, color::Color};

use crate::{handler::MyWindowHandler, structs::{Layer, Atom}};


pub fn start(w: &mut MyWindowHandler, helper: &mut WindowHelper<()>, _info: WindowStartupInfo) {
	// load window icon
	let path = "assets/icon.png";
	match crate::util::load_png(path) {
		Ok((icon, size)) => {
			match helper.set_icon_from_rgba_pixels(icon, size) {
				Ok(_) => (),
				Err(error) => println!("Couldn't set window icon: {:?}", error)
			}
		}
		Err(_) => println!("icon.png was not found at {path}")
	};
	
	// load font
	w.font1 = Some(Font::new(include_bytes!("../assets/fonts/NotoSans-Regular.ttf")).unwrap());
	w.str1 = String::from("text");
	//self.text1 = construct_text_option(&self.str1, &self.font1, 64.0);
	
	// initialize atoms
	let pd_color = Color::from_rgb(0.1, 0.7, 0.7);
	let se_color = Color::from_rgb(0.1, 0.8, 0.1);
	let s_color = Color::from_rgb(0.7, 0.7, 0.1);
	let w_color = Color::from_rgb(0.1, 0.5, 0.9);
	let mo_color = Color::from_rgb(0.6, 0.1, 0.9);
	//let bond_color = Color::from_rgb(0.7, 0.7, 0.7);
	
	let r = 0.5;
	let size: usize = 200;
	
	// PdSe2
	let mut pdse2 = Layer { atoms: Vec::new(), bonds: Vec::new(), angle: 0.0, x: 0.0, y: 0.0, z: 0.0, name: String::from("PdSe2"), show: true };
	let a = 5.755;
	let b = 5.901;
	
	let mut x_shift = 0.0;
	for _ in 0..size {
		let mut y_shift = 0.0;
		for _ in 0..size {
			pdse2.atoms.append(&mut vec![
				Atom { x: x_shift, y: y_shift, z: 0.0, r, color: pd_color },
				Atom { x: x_shift + a*0.5, y: y_shift + b*0.5, z: 0.0, r, color: pd_color },
				Atom { x: x_shift + a*0.1105, y: y_shift + b*0.3813, z: -0.731, r, color: se_color },
				Atom { x: x_shift + a*0.3896, y: y_shift + b*(1.0-0.1186), z: -0.731, r, color: se_color },
				Atom { x: x_shift + a*(1.0-0.1105), y: y_shift + b*(1.0-0.3813), z: 0.731, r, color: se_color },
				Atom { x: x_shift + a*(1.0-0.3896), y: y_shift + b*0.1186, z: 0.731, r, color: se_color },
			]);
			
			y_shift += b;
		}
		x_shift += a;
	}
	
	
	// PdSe2 rotated
	let mut pdse2r = pdse2.clone();
	pdse2r.name = String::from("PdSe2 Rotated");
	pdse2r.z = 10.0;
	
	
	w.layers = vec![pdse2, pdse2r];
	
	
	// Hexagonal TMDs
	
	for ( name		, a		, c		, color1	, color2	, z		) in [
		( "WSe2"	, 3.32	, 1.662	, w_color	, se_color	, 20.0	),
		( "MoSe2"	, 3.322	, 1.656	, mo_color	, se_color	, 30.0	),
		( "WS2"		, 3.184	, 1.56	, w_color	, s_color	, 40.0	),
		( "MoS2"	, 3.193	, 1.565	, mo_color	, s_color	, 50.0	),
	] {
		let mut layer = Layer { atoms: Vec::new(), bonds: Vec::new(), angle: 0.0, x: 0.0, y: 0.0, z, name: String::from(name), show: true };
		
		let dx = a * -0.5;
		let dy = a * f64::sqrt(3.0) * 0.5;
		for i in 0..size*2 {
			let mut x_shift = i as f64 * a;
			let mut y_shift = 0.0;
			for _ in 0..size*2 {
				layer.atoms.append(&mut vec![
					Atom { x: x_shift + a/3.0 + dx*2.0/3.0, y: y_shift + dy*2.0/3.0, z: 0.0, r, color: color1 },
					Atom { x: x_shift + a*2.0/3.0 + dx/3.0, y: y_shift + dy/3.0, z: c, r, color: color2 },
					//Atom { x: x_shift + a*2.0/3.0 + dx/3.0, y: y_shift + dy/3.0, z: -c, r, color: color2 },
				]);
				
				x_shift += dx;
				y_shift += dy;
			}
		}
		
		w.layers.push(layer);
	}
	
}

