
extern crate system_register_protocol;

use std::env;

use system_register_protocol::{ open };

fn main() {

	if let Some(url) = env::args().skip(1).next() {
		println!("Trying to open {}", url);
		open(url);
	} else {
		println!("Please specify your URL")
	}

}