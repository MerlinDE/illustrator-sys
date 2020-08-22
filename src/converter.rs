use crate::root as ai_sys;
use std::convert::From;
use std::ffi::{CStr, CString};

#[no_mangle]
impl From<ai_sys::ai::UnicodeString> for String {
    fn from(item: ai_sys::ai::UnicodeString) -> Self {
        // let s = ai_sys::std::basic_string::from(item.);
        unsafe {
            String::from(
                CString::from_raw(item.as_UTF8().as_mut_ptr() as *mut i8)
                    .to_str()
                    .unwrap(),
            )
        }
    }
}

#[no_mangle]
impl From<String> for ai_sys::ai::UnicodeString {
    fn from(item: String) -> Self {
        let s = CString::new(item.as_str()).unwrap();
        unsafe { ai_sys::ai::UnicodeString_FromUTF8(s.as_bytes_with_nul().as_ptr() as *const i8) }
    }
}

// impl From<ai_sys::std::string> for String {
//     fn from(item: ai_sys::std::string) -> Self {
//         let s = unsafe { ai_sys::ai::UnicodeString_FromUTF8(item) };
//     }
// }
