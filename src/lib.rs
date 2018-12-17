#![feature(uniform_paths)] 

#[cfg(target_os = "linux")]
mod lib_linux;
#[cfg(target_os = "linux")]
pub use lib_linux::*;
