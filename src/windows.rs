extern crate winreg;


use std::path::Path;
use std::process::Command;
use self::winreg::RegKey;
use self::winreg::enums::HKEY_CURRENT_USER;

use app::App;

// as described at https://msdn.microsoft.com/en-us/library/aa767914(v=vs.85).aspx


pub fn install(app: App, schemes: Vec<String>) -> bool {

	// but we can't write on root, we'll have to do it for the curent user only
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for protocol in schemes {
	    let base_path = Path::new("Software").join("Classes").join(protocol);
	    let key = hkcu.create_subkey(&base_path).unwrap();
	    // set our app name as the for reference
	    key.set_value("", &app.name).unwrap();
	    //
	    key.set_value("URL Protocol", &"").unwrap();

	    let command_key = hkcu.create_subkey(&base_path.join("shell").join("open").join("command")).unwrap();
	    command_key.set_value("", &format!("\"{}\" \"%1\"", app.exec)).unwrap()
    }
    true
}


pub fn open(url: String) -> bool {
	Command::new("explorer")
				 .arg(url)
                 .status()
                 .expect("failed to run 'start'");
    // 'explorer' always comes back witha  bad error code :(
    // but neither 'start' nor 'cmd /c start' seem to work...
    true
}
