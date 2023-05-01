
use std::{f64::consts::PI, io::BufWriter, fs::File};
use speedy2d::{window::WindowHelper, Graphics2D, color::Color, image::ImageDataType};

use crate::{handler::MyWindowHandler, util::construct_text_option};


pub fn draw(w: &mut MyWindowHandler, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
	// measure framerate
	let now = w.frame_timer.elapsed();
	let dt = (now - w.previous_frame_timestamp).as_nanos() as f64 * 1e-9;
	let avg_proportion: f64 = 0.05;
	w.frame_time_avg = w.frame_time_avg * (1.0 - avg_proportion) + dt * avg_proportion;
	w.previous_frame_timestamp = now;
	
	// show framerate
	let _fps_str = ((1.0 / w.frame_time_avg) as u32).to_string() + " fps";
	
	// show mouse position
	let mouse_pos = w.input(w.mouse_x, w.mouse_y);
	let mouse_pos_str = format!("{:.3}, {:.3}   ({:.3}A, {:.3}°)", mouse_pos.0, mouse_pos.1, f64::sqrt(mouse_pos.0 * mouse_pos.0 + mouse_pos.1 * mouse_pos.1), f64::atan2(mouse_pos.1, mouse_pos.0) * 180.0 / PI);
	w.text2 = construct_text_option(&mouse_pos_str, &w.font1, 48.0);
	
	// show angle
	w.text1 = construct_text_option(format!("{:.3}°", w.layers[1].angle).as_str(), &w.font1, 48.0);
	
	
	
	let width = w.size.x as f64;
	let height = w.size.y as f64;
	let output_scale = width / (w.fov * 2.0);
	
	graphics.clear_screen(Color::from_rgb(0.05, 0.05, 0.05));
	
	//w.atoms.sort_by(|a, b| )
	
	//for bond in &w.bonds {
	//	graphics.draw_line(w.atom_output(&w.atoms[bond.index1]), w.atom_output(&w.atoms[bond.index2]), 0.2 * output_scale, bond.color)
	//}
	
	for layer in &w.layers {
		if !layer.show {
			continue
		}
		
		let t = w.get_output_transform(layer.angle);
		for atom in &layer.atoms {
			graphics.draw_circle((((atom.x + layer.x)*t.0 + (atom.y + layer.y)*t.2 + (atom.z + layer.z)*t.4 + t.6) as f32, ((atom.x + layer.x)*t.1 + (atom.y + layer.y)*t.3 + (atom.z + layer.z)*t.5 + t.7) as f32), (atom.r * output_scale) as f32, atom.color);
		}
	}
	
	if let Some(text) = &w.text1 {
		graphics.draw_text((20.0, text.height() - 40.0), Color::WHITE, &text);
	}
	if let Some(text) = &w.text2 {
		graphics.draw_text((20.0, height as f32 - text.height() - 20.0), Color::WHITE, &text);
	}
	
	
	if w.screenshot {
		graphics.draw_circle((w.mouse_x, w.mouse_y), 5.0, Color::MAGENTA);
		let image_data = graphics.capture(ImageDataType::RGB);
		let mut encoder = png::Encoder::new(BufWriter::new(File::create(format!("captures/{:.3}° {:.3}A.png", w.layers[1].angle, f64::sqrt(mouse_pos.0 * mouse_pos.0 + mouse_pos.1 * mouse_pos.1))).unwrap()), w.size.x, w.size.y);
		encoder.set_color(png::ColorType::RGB);
		encoder.write_header().unwrap().write_image_data(image_data.data()).unwrap();
		
		w.screenshot = false;
	}
	
	
	helper.request_redraw();
}



