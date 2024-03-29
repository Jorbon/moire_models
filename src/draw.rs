
use std::{io::BufWriter, fs::File};
use speedy2d::{window::WindowHelper, Graphics2D, color::Color, image::ImageDataType};

use crate::{handler::{MyWindowHandler, Lattice}, util::construct_text};


fn draw_pdse2(graphics: &mut Graphics2D, a: f64, b: f64, pd_color: Color, se_color: Color, bond_color: Color, r: f64, bond_r: f64, show_atoms: bool, show_bonds: bool, start_x: i32, start_y: i32, end_x: i32, end_y: i32, t: (f64, f64, f64, f64, f64, f64, f64), bilayer_shift: bool) {
	if (end_x - start_x) * (end_y - start_y) > 150_000 {
		return
	}
	
	let x1 = 0.1105*a;
	let x2 = 0.3896*a;
	let y1 = 0.3813*b;
	let y2 = 0.1186*b;
	let points = [
		(0.0, 0.0, pd_color),
		(0.5*a, 0.5*b, pd_color),
		(x1, y1, se_color),
		(x2, b-y2, se_color),
		(a-x1, b-y1, se_color),
		(a-x2, y2, se_color),
	];
	
	for i in start_x..=end_x {
		let x_shift = a * (i as f64 + match bilayer_shift { true => 0.5, false => 0.0 });
		
		for j in start_y..=end_y {
			let y_shift = j as f64 * b;
			
			if show_bonds {
				for (i1, i2, ox, oy) in [
					(1, 2, 0.0, 0.0),
					(1, 3, 0.0, 0.0),
					(1, 4, 0.0, 0.0),
					(1, 5, 0.0, 0.0),
					(2, 0, 0.0, 0.0),
					(3, 0, 0.0, b),
					(4, 0, a, b),
					(5, 0, a, 0.0),
					(3, 5, 0.0, b),
					(4, 2, a, 0.0),
				] {
					graphics.draw_line(
						(((points[i1].0 + x_shift)*t.0 + (points[i1].1 + y_shift)*t.2 + t.4) as f32,
						((points[i1].0 + x_shift)*t.1 + (points[i1].1 + y_shift)*t.3 + t.5) as f32),
						(((points[i2].0 + ox + x_shift)*t.0 + (points[i2].1 + oy + y_shift)*t.2 + t.4) as f32,
						((points[i2].0 + ox + x_shift)*t.1 + (points[i2].1 + oy + y_shift)*t.3 + t.5) as f32),
						(bond_r * t.6) as f32,
						bond_color
					);
				}
			}
			
			if show_atoms {
				for (x, y, color) in points {
					graphics.draw_circle((((x + x_shift)*t.0 + (y + y_shift)*t.2 + t.4) as f32, ((x + x_shift)*t.1 + (y + y_shift)*t.3 + t.5) as f32), (r * t.6) as f32, color);
				}
			}
			
		}
	}
}

fn draw_crocl(graphics: &mut Graphics2D, a: f64, b: f64, cr_color: Color, o_color: Color, cl_color: Color, bond_color: Color, r: f64, bond_r: f64, show_atoms: bool, show_bonds: bool, start_x: i32, start_y: i32, end_x: i32, end_y: i32, t: (f64, f64, f64, f64, f64, f64, f64), bilayer_shift: bool) {
	if (end_x - start_x) * (end_y - start_y) > 200_000 {
		return
	}
	
	for i in start_x..=end_x {
		let x_shift = a * (i as f64 + match bilayer_shift { true => 0.5, false => 0.0 });
		
		for j in start_y..=end_y {
			let y_shift = j as f64 * b;
			
			if show_bonds {
				graphics.draw_line(
					((x_shift*t.0 + y_shift*t.2 + t.4) as f32,
					(x_shift*t.1 + y_shift*t.3 + t.5) as f32),
					(((x_shift + a)*t.0 + y_shift*t.2 + t.4) as f32,
					((x_shift + a)*t.1 + y_shift*t.3 + t.5) as f32),
					(bond_r * t.6) as f32,
					bond_color
				);
				graphics.draw_line(
					((x_shift*t.0 + (y_shift + 0.5*b)*t.2 + t.4) as f32,
					(x_shift*t.1 + (y_shift + 0.5*b)*t.3 + t.5) as f32),
					(((x_shift + a)*t.0 + (y_shift + 0.5*b)*t.2 + t.4) as f32,
					((x_shift + a)*t.1 + (y_shift + 0.5*b)*t.3 + t.5) as f32),
					(bond_r * t.6) as f32,
					bond_color
				);
				graphics.draw_line(
					((x_shift*t.0 + y_shift*t.2 + t.4) as f32,
					(x_shift*t.1 + y_shift*t.3 + t.5) as f32),
					((x_shift*t.0 + (y_shift + b)*t.2 + t.4) as f32,
					(x_shift*t.1 + (y_shift + b)*t.3 + t.5) as f32),
					(bond_r * t.6) as f32,
					bond_color
				);
				graphics.draw_line(
					(((x_shift + 0.5*a)*t.0 + y_shift*t.2 + t.4) as f32,
					((x_shift + 0.5*a)*t.1 + y_shift*t.3 + t.5) as f32),
					(((x_shift + 0.5*a)*t.0 + (y_shift + b)*t.2 + t.4) as f32,
					((x_shift + 0.5*a)*t.1 + (y_shift + b)*t.3 + t.5) as f32),
					(bond_r * t.6) as f32,
					bond_color
				);
			}
			
			if show_atoms {
				for (x, y, color) in [
					(0.0, 0.0, cr_color),
					(0.5*a, 0.5*b, cr_color),
					(0.5*a, 0.0, o_color),
					(0.0, 0.5*b, cl_color)
				] {
					graphics.draw_circle((((x + x_shift)*t.0 + (y + y_shift)*t.2 + t.4) as f32, ((x + x_shift)*t.1 + (y + y_shift)*t.3 + t.5) as f32), (r * t.6) as f32, color);
				}
			}
			
		}
	}
}

fn draw_tmd(graphics: &mut Graphics2D, a: f64, tm_color: Color, cg_color: Color, bond_color: Color, r: f64, bond_r: f64, show_atoms: bool, show_bonds: bool, start_x: i32, start_y: i32, end_x: i32, end_y: i32, t: (f64, f64, f64, f64, f64, f64, f64)) {
	if (end_x - start_x) * (end_y - start_y) > 200_000 {
		return
	}
	
	let dx = a * -0.5;
	let dy = a * f64::sqrt(3.0) * 0.5;
	for i in start_x..=end_x {
		let mut x_shift = i as f64 * a + start_y as f64 * dx;
		let mut y_shift = start_y as f64 * dy;
		for _ in start_y..=end_y {
			let dx1 = x_shift + a/3.0 + dx*2.0/3.0;
			let dy1 = y_shift + dy*2.0/3.0;
			let dx2 = x_shift + a*2.0/3.0 + dx/3.0;
			let dy2 = y_shift + dy/3.0;
			if show_bonds {
				graphics.draw_line(
					((dx1*t.0 + dy1*t.2 + t.4) as f32,
					(dx1*t.1 + dy1*t.3 + t.5) as f32),
					((dx2*t.0 + dy2*t.2 + t.4) as f32,
					(dx2*t.1 + dy2*t.3 + t.5) as f32),
					(bond_r * t.6) as f32,
					bond_color
				);
				graphics.draw_line(
					((dx1*t.0 + dy1*t.2 + t.4) as f32,
					(dx1*t.1 + dy1*t.3 + t.5) as f32),
					(((dx2 + dx)*t.0 + (dy2 + dy)*t.2 + t.4) as f32,
					((dx2 + dx)*t.1 + (dy2 + dy)*t.3 + t.5) as f32),
					(bond_r * t.6) as f32,
					bond_color
				);
				graphics.draw_line(
					(((dx1 + a)*t.0 + dy1*t.2 + t.4) as f32,
					((dx1 + a)*t.1 + dy1*t.3 + t.5) as f32),
					((dx2*t.0 + dy2*t.2 + t.4) as f32,
					(dx2*t.1 + dy2*t.3 + t.5) as f32),
					(bond_r * t.6) as f32,
					bond_color
				);
			}
			if show_atoms {
				graphics.draw_circle(((dx1*t.0 + dy1*t.2 + t.4) as f32, (dx1*t.1 + dy1*t.3 + t.5) as f32), (r * t.6) as f32, tm_color);
				graphics.draw_circle(((dx2*t.0 + dy2*t.2 + t.4) as f32, (dx2*t.1 + dy2*t.3 + t.5) as f32), (r * t.6) as f32, cg_color);
			}
			x_shift += dx;
			y_shift += dy;
		}
	}
}


pub fn draw(w: &mut MyWindowHandler, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
	// measure framerate
	let now = w.frame_timer.elapsed();
	let dt = (now - w.previous_frame_timestamp).as_nanos() as f64 * 1e-9;
	let avg_proportion = dt * 3.0;
	w.frame_time_avg = w.frame_time_avg * (1.0 - avg_proportion) + dt * avg_proportion;
	w.previous_frame_timestamp = now;
	
	
	
	let width = w.size.x as f32;
	let height = w.size.y as f32;
	
	graphics.clear_screen(Color::from_rgb(0.05, 0.05, 0.05));
	
	
	if w.show_fixed {
		let (start_x, start_y) = w.input(0.0, 0.0);
		let (end_x, end_y) = w.input(width as f32, height as f32);
		
		match w.fixed_lattice {
			Lattice::PdSe2(a, b, pd, se) => {
				draw_pdse2(graphics, a, b, pd, se, w.bond_color, w.r, w.bond_r, w.show_atoms, w.show_bonds,
					(start_x / a) as i32 - 1,
					(start_y / b) as i32 - 1,
					(end_x / a) as i32,
					(end_y / b) as i32,
					w.get_output_transform(0.0),
					false
				);
				if w.bilayer && w.static_bilayer {
					draw_pdse2(graphics, a, b, pd, se, w.bond_color, w.r, w.bond_r, w.show_atoms, w.show_bonds,
						(start_x / a) as i32 - 1,
						(start_y / b) as i32 - 1,
						(end_x / a) as i32,
						(end_y / b) as i32,
						w.get_output_transform(0.0),
						true
					);
				}
			}
			Lattice::CrOCl(a, b, cr, o, cl) => {
				draw_crocl(graphics, a, b, cr, o, cl, w.bond_color, w.r, w.bond_r, w.show_atoms, w.show_bonds,
					(start_x / a) as i32 - 1,
					(start_y / b) as i32 - 1,
					(end_x / a) as i32,
					(end_y / b) as i32,
					w.get_output_transform(0.0),
					false
				);
			}
			Lattice::TMD(a, tm, cg) => {
				draw_tmd(graphics, a, tm, cg, w.bond_color, w.r, w.bond_r, w.show_atoms, w.show_bonds,
					((start_x + start_y / f64::sqrt(3.0)) / a) as i32 - 1,
					(start_y * 2.0 / (a * f64::sqrt(3.0))) as i32 - 1,
					((end_x + end_y / f64::sqrt(3.0)) / a) as i32,
					(end_y * 2.0 / (a * f64::sqrt(3.0))) as i32,
					w.get_output_transform(0.0)
				);
			}
		};
	}
	
	
	if w.show_rotated {
		let mut minx = f64::MAX;
		let mut miny = f64::MAX;
		let mut maxx = f64::MIN;
		let mut maxy = f64::MIN;
		
		for (x, y) in [
			(0.0, 0.0),
			(width, 0.0),
			(0.0, height),
			(width, height)
		] {
			let (inx, iny) = w.input_rotated(x, y);
			minx = f64::min(minx, inx);
			miny = f64::min(miny, iny);
			maxx = f64::max(maxx, inx);
			maxy = f64::max(maxy, iny);
		}
		
		match w.rotated_lattice {
			Lattice::PdSe2(a, b, pd, se) => {
				draw_pdse2(graphics, a, b, pd, se, w.bond_color, w.r, w.bond_r, w.show_atoms, w.show_bonds,
					(minx / a) as i32 - 1,
					(miny / b) as i32 - 1,
					(maxx / a) as i32,
					(maxy / b) as i32,
					w.get_output_transform(w.angle),
					false
				);
				if w.bilayer {
					draw_pdse2(graphics, a, b, pd, se, w.bond_color, w.r, w.bond_r, w.show_atoms, w.show_bonds,
						(minx / a) as i32 - 1,
						(miny / b) as i32 - 1,
						(maxx / a) as i32,
						(maxy / b) as i32,
						w.get_output_transform(w.angle),
						true
					);
				}
			}
			Lattice::CrOCl(a, b, cr, o, cl) => {
				draw_crocl(graphics, a, b, cr, o, cl, w.bond_color, w.r, w.bond_r, w.show_atoms, w.show_bonds,
					(minx / a) as i32 - 1,
					(miny / b) as i32 - 1,
					(maxx / a) as i32,
					(maxy / b) as i32,
					w.get_output_transform(w.angle),
					false
				);
				if w.bilayer {
					draw_crocl(graphics, a, b, cr, o, cl, w.bond_color, w.r, w.bond_r, w.show_atoms, w.show_bonds,
						(minx / a) as i32 - 1,
						(miny / b) as i32 - 1,
						(maxx / a) as i32,
						(maxy / b) as i32,
						w.get_output_transform(w.angle),
						true
					);
				}
			}
			Lattice::TMD(..) => ()
		}
	}
	
	
	let mouse_pos = w.input(w.mouse_x, w.mouse_y);
	
	if w.show_overlay {
		let mut scale = 1.0;
		let mut scale_str = "1A";
		let strs = ["1A", "1nm", "10nm", "100nm", "1μm", "10μm", "100μm", "1mm", "10mm", "100mm", "1m"];
		for i in (1..=10_u32).rev() {
			let s = usize::pow(10, i) as f64;
			if w.fov * 1.5 > s {
				scale = s;
				scale_str = strs[i as usize];
				break;
			}
		}
		
		let output_scale = w.get_output_scale();
		
		let left_x = (width - (scale * output_scale) as f32) * 0.5;
		let right_x = (width + (scale * output_scale) as f32) * 0.5;
		graphics.draw_line((left_x, 20.0), (right_x, 20.0), 3.0, Color::WHITE);
		graphics.draw_line((left_x, 20.0), (left_x, 50.0), 3.0, Color::WHITE);
		graphics.draw_line((right_x, 20.0), (right_x, 50.0), 3.0, Color::WHITE);
		
		let scale_text = construct_text(scale_str, &w.font, 40.0);
		graphics.draw_text(((width - scale_text.width()) * 0.5, 25.0), Color::WHITE, &scale_text);
		
		let fps_text = construct_text(format!("{:.0} fps", 1.0 / w.frame_time_avg).as_str(), &w.font, 48.0);
		graphics.draw_text((20.0, 20.0), Color::WHITE, &fps_text);
		
		let angle_text = construct_text(format!("{:.3}°", w.angle).as_str(), &w.font, 48.0);
		graphics.draw_text((20.0, height as f32 - angle_text.height() - 20.0), Color::WHITE, &angle_text);
		
		let mouse_pos_text = construct_text(format!("{:.3}, {:.3}", mouse_pos.0, mouse_pos.1).as_str(), &w.font, 48.0);
		graphics.draw_text((width - mouse_pos_text.width() - 20.0, height - mouse_pos_text.height() - 20.0), Color::WHITE, &mouse_pos_text);
	}
	
	
	if w.screenshot {
		if w.screenshot_dot {
			graphics.draw_circle((w.mouse_x, w.mouse_y), 5.0, Color::MAGENTA);
		}
		let image_data = graphics.capture(ImageDataType::RGB);
		let mut encoder = png::Encoder::new(BufWriter::new(File::create(format!("captures/{:.3}° ({:.3}, {:.3}).png", w.angle, mouse_pos.0, mouse_pos.1)).unwrap()), w.size.x, w.size.y);
		encoder.set_color(png::ColorType::RGB);
		encoder.write_header().unwrap().write_image_data(image_data.data()).unwrap();
		
		w.screenshot = false;
	}
	
	
	helper.request_redraw();
}



