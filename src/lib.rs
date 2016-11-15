

// inspired by https://github.com/feross/webtorrent-desktop/blob/4bb2056bc9c1a421815b97d03ffed512575dfde0/src/main/handlers.js

#[derive(Debug)]
pub struct App {
	// path to execute, including optional parameters
	pub exec: String,
	// the display name of the application
	pub name: String,
	// an optional icon, only supported on some platforms
	pub icon: Option<String>,
}


#[cfg(target_os = "linux")]
pub fn install(app: App) {
    println!("Hello from {:?} for Linux", app.exec);
}



#[cfg(target_os = "macos")]
pub fn install(app: App) {

} 



#[cfg(target_os = "windows")]
pub fn install(app: App) {

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
