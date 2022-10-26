use colorize::*;
use std::panic;
// use crate::*;

pub fn override_panic() {
	panic::set_hook(Box::new(|panic_info| {
		// panic_info.
		if let Some(location) = panic_info.location() {
			let msg = if let Some(s) = panic_info.payload().downcast_ref::<String>() {
				s
			} else {
				"unknown error"
			};

			let file = location.file();
			let line = location.line();
			// location.
			// let root = get_project_root().unwrap_or_default();
			// let name = root.display().to_string();
			// let relative_file = file.replace(&name, "");
			// println!("{}",root.display());


			//this is the wrong trace!
			let trace = ["at ",file,":",&line.to_string()].concat().faint();
			println!(
				"{}\n\n{}\n\n{}\n",
				"ERROR".red(),
				msg,
				trace
			);
		} else {
			// println!("{}", panic_info.to_string());
			// println!("panic occurred but can't get location information...");
		}
	}));
}
