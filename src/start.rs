
use speedy2d::{window::{WindowHelper, WindowStartupInfo}, font::Font};

use crate::handler::MyWindowHandler;


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
	
	
}

