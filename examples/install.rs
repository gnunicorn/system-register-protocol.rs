
extern crate system_register_protocol;

use system_register_protocol::{ App, install };

fn main() {

	let exec = String::from(std::env::current_exe().unwrap().to_str().unwrap());
	let app = App {exec: exec, name: "Example".to_string(), icon: None};
	install(app)
}