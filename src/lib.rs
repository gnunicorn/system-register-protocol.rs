

// inspired by https://github.com/feross/webtorrent-desktop/blob/4bb2056bc9c1a421815b97d03ffed512575dfde0/src/main/handlers.js


mod app;
pub use app::{App}; 



#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{ install };



#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{ install };




#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{ install };




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
