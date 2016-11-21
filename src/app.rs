

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


impl App {
    pub fn new(bundle_id: String,
               vendor: String,
               name: String,
               exec: String,
               icon: Option<String>)
               -> Self {
        App {
            bundle_id: bundle_id,
            name: name,
            vendor: vendor,
            exec: exec,
            icon: icon,
        }
    }
}
