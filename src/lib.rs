#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(clippy::all)]
#![allow(improper_ctypes)]

pub use converter::*;

// Let's hope the authors of `bindgen` know better ;)
include!(concat!(env!("OUT_DIR"), "/illustrator-sys.rs"));
//include!(concat!(env!("OUT_DIR"), "/wrapper.rs"));
pub mod converter;
