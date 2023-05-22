
use speedy2d::window::{WindowHelper, WindowStartupInfo};

use crate::handler::MyWindowHandler;


pub fn start(_w: &mut MyWindowHandler, helper: &mut WindowHelper<()>, _info: WindowStartupInfo) {
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
	
}

