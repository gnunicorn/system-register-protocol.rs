

#[derive(Debug)]
pub struct App {
	// our apps bundle_id
	pub bundle_id: String,
	// path to execute, including optional parameters
	pub exec: String,
	// What's the vendor?
	pub vendor: String,
	// the display name of the application
	pub name: String,
	// an optional icon, only supported on some platforms
	pub icon: Option<String>,
}
