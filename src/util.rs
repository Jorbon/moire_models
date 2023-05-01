
use std::{rc::Rc, fs::File};
use speedy2d::{font::{Font, TextOptions, FormattedTextBlock, TextLayout}, dimen::Vector2};


pub fn _construct_text(text: &str, font: &Font, size: f32) -> Rc<FormattedTextBlock> {
	font.layout_text(text, size, TextOptions::new())
}

pub fn construct_text_option(text: &str, font: &Option<Font>, size: f32) -> Option<Rc<FormattedTextBlock>> {
	Some(font.as_ref().unwrap().layout_text(text, size, TextOptions::new()))
}

pub fn load_png(path: &str) -> Result<(Vec<u8>, Vector2<u32>), std::io::Error> {
	match File::open(path) {
		Ok(file) => {
			let decoder = png::Decoder::new(file);
			let (info, mut reader) = decoder.read_info().unwrap();
			let mut buf = vec![0; info.buffer_size()];
			reader.next_frame(&mut buf).unwrap();
			
			Ok((buf, Vector2::new(info.width, info.height)))
		}
		Err(val) => Err(val)
	}
	
	//let bytes = &buf[..info.buffer_size()];
	//let in_animation = reader.info().frame_control.is_some();
}



