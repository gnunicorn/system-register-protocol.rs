

use std::env;
use std::ascii::AsciiExt;
use std::fs::{ create_dir_all, File};
use std::io::Write;
use std::process::Command;
use std::path::PathBuf;

use app::App;

pub fn open(url: String) -> bool {
	Command::new("xdg-open")
                 .arg(url)
                 .status()
                 .expect("failed to start xdg-open"
    ).success()
}

pub fn install(app: App, schemes: Vec<String>) -> bool {
	if let Some(home) = env::home_dir() {
    	let ascii_name = format!("{}-{}",
    			app.vendor.as_str().to_ascii_lowercase(),
    			app.name.as_str().to_ascii_lowercase());
    	let mut desktop_target = PathBuf::new();
    	desktop_target.push(home);
    	desktop_target.push(".local");
    	desktop_target.push("share");
    	desktop_target.push("applications");
    	let apps_dir = desktop_target.clone();
    	if let Ok(_) = create_dir_all(apps_dir.clone()) {
    		desktop_target.push(ascii_name + ".desktop");
	    	if let Ok(mut f) = File::create(desktop_target.as_path()) {
	    		if let Ok(_) = f.write_fmt(format_args!("[Desktop Entry]
Type=Application
Name={}
Exec={} %u
Terminal=false
MimeType={}
NoDisplay=true
",
	app.name,
	app.exec,
	// app.icon.unwrap_or("".to_string()),	
	schemes.iter().map( |s| format!("x-scheme-handler/{};", s) ).collect::<Vec<String>>().join(""))
	 			) {
					Command::new("update-desktop-database")
				                 .arg(apps_dir)
				                 .status()
				                 .expect("failed to run update-desktop-database"
				    ).success()
	    		} else {
	    			false
	    		}
	    	} else {
	    		false
	    	}
    	} else {
    		false
    	}
	} else {
		false
	}
}
