
extern crate system_register_protocol;
extern crate rand;

use rand::Rng;
use std::env;
use std::process::exit;


use system_register_protocol::{ App, install, open };

fn install_and_open() -> bool {

	let mut rng = rand::thread_rng();
	let exec = String::from(std::env::current_exe().unwrap().to_str().unwrap());
	let app = App {exec: exec,
				   vendor: "MaidSafe".to_string(),
				   name: "Example".to_string(),
				   icon: None};
	let schema = format!("testschema{}", rng.gen::<u32>());

	println!("Installing ourselves under {}", schema);


	if install(app, vec![schema.clone()]) {

		println!("Install succeeded 😄");
		println!("Trying to open {}:test", schema);

		if open(format!("{}:test", schema)) {
			println!("Open succeeded 😄, everything is fine 🎉!");
			true
		} else {
			println!("⚠ Opening failed 😟");
			false
		}

	} else {
		println!("⚠ Install failed 😟");
		false
	}

}

fn main() {

	if let Some(url) = env::args().skip(1).next() {
		println!("Being started with {} as first parameter. Yay 🎉 and good bye", url);
		exit(0);
	}

	if !install_and_open() {
		exit(1)
	}

}