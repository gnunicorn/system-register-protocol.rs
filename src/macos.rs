

use std::process::Command;

use app::App;

use libc::{c_void, c_long);



#[repr(C)]
struct __CFString(c_void);
type CFStringRef = *const __CFString;

type CFAllocatorRef = *const c_void;
type CFIndex = c_long;
type CFStringEncoding = u32;


#[link(name = "CoreFoundation", kind = "framework")]
extern {
	static kCFAllocatorDefault: CFAllocatorRef;
	static kCFAllocatorNull: CFAllocatorRef;
    fn CFStringCreateWithBytes(alloc: CFAllocatorRef,
                               bytes: *const u8,
                               numBytes: CFIndex,
                               encoding: CFStringEncoding,
                               isExternalRepresentation: u8,
                               contentsDeallocator: CFAllocatorRef)
                               -> CFStringRef;
}


#[link(name = "CoreServices", kind = "framework")]
extern {
    fn LSSetDefaultHandlerForURLScheme(scheme: CFStringRef, bundle_id:CFStringRef);
}


// helper to hand over strings to macos
fn convert_to_cfstring(content: &str) -> CFStringRef {
    unsafe {
		CFStringCreateWithBytes(kCFAllocatorDefault,
                                content.as_ptr(),
                                content.len() as CFIndex,
                                0x08000100 as CFStringEncoding,
                                false as u8,
                                kCFAllocatorNull)
    }
}




pub fn open(url: String) -> bool {
	Command::new("open")
                 .arg(url)
                 .status()
                 .expect("failed to start open"
    ).success()
}

pub fn install(app: App, schemes: Vec<String>) -> bool {
    let bundle_id = convert_to_cfstring(App.bundle_id.to_str());
    for scheme in schemes {
    	// FIXME: do we have any way to learn this failed?
    	unsafe { LSSetDefaultHandlerForURLScheme(convert_to_cfstring(scheme), bundle_id); }
    }

    true
}