use std::convert::{From, TryFrom};
use std::ffi::CString;

use crate::root as ai_sys;

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

// impl From<ai_sys::AIBoolean> for bool {
//     fn from(item: ai_sys::AIBoolean) -> Self {
//         return item != 0;
//     }
// }

// TODO: find a better way to do this stupid conversion!!!!
impl TryFrom<i16> for ai_sys::AIArtType {
    type Error = ();

    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            x if x == ai_sys::AIArtType::kAnyArt as i16 => Ok(ai_sys::AIArtType::kAnyArt),
            x if x == ai_sys::AIArtType::kUnknownArt as i16 => Ok(ai_sys::AIArtType::kUnknownArt),
            x if x == ai_sys::AIArtType::kGroupArt as i16 => Ok(ai_sys::AIArtType::kGroupArt),
            x if x == ai_sys::AIArtType::kPathArt as i16 => Ok(ai_sys::AIArtType::kPathArt),
            x if x == ai_sys::AIArtType::kCompoundPathArt as i16 => {
                Ok(ai_sys::AIArtType::kCompoundPathArt)
            }
            x if x == ai_sys::AIArtType::kTextArtUnsupported as i16 => {
                Ok(ai_sys::AIArtType::kTextArtUnsupported)
            }
            x if x == ai_sys::AIArtType::kTextPathArtUnsupported as i16 => {
                Ok(ai_sys::AIArtType::kTextPathArtUnsupported)
            }
            x if x == ai_sys::AIArtType::kTextRunArtUnsupported as i16 => {
                Ok(ai_sys::AIArtType::kTextRunArtUnsupported)
            }
            x if x == ai_sys::AIArtType::kPlacedArt as i16 => Ok(ai_sys::AIArtType::kPlacedArt),
            x if x == ai_sys::AIArtType::kMysteryPathArt as i16 => {
                Ok(ai_sys::AIArtType::kMysteryPathArt)
            }
            x if x == ai_sys::AIArtType::kRasterArt as i16 => Ok(ai_sys::AIArtType::kRasterArt),
            x if x == ai_sys::AIArtType::kPluginArt as i16 => Ok(ai_sys::AIArtType::kPluginArt),
            x if x == ai_sys::AIArtType::kMeshArt as i16 => Ok(ai_sys::AIArtType::kMeshArt),
            x if x == ai_sys::AIArtType::kTextFrameArt as i16 => {
                Ok(ai_sys::AIArtType::kTextFrameArt)
            }
            x if x == ai_sys::AIArtType::kSymbolArt as i16 => Ok(ai_sys::AIArtType::kSymbolArt),
            x if x == ai_sys::AIArtType::kForeignArt as i16 => Ok(ai_sys::AIArtType::kForeignArt),
            x if x == ai_sys::AIArtType::kLegacyTextArt as i16 => {
                Ok(ai_sys::AIArtType::kLegacyTextArt)
            }
            x if x == ai_sys::AIArtType::kChartArt as i16 => Ok(ai_sys::AIArtType::kChartArt),
            x if x == ai_sys::AIArtType::kRadialRepeatArt as i16 => {
                Ok(ai_sys::AIArtType::kRadialRepeatArt)
            }
            x if x == ai_sys::AIArtType::kGridRepeatArt as i16 => {
                Ok(ai_sys::AIArtType::kGridRepeatArt)
            }
            x if x == ai_sys::AIArtType::kSymmetryArt as i16 => Ok(ai_sys::AIArtType::kSymmetryArt),
            x if x == ai_sys::AIArtType::kConcentricRepeatArt as i16 => {
                Ok(ai_sys::AIArtType::kConcentricRepeatArt)
            }
            _ => Err(()), // TODO: return correct error code
        }
    }
}

// TODO: rewrite into general conversion macro
// taken from https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer
#[allow(unused_macros)]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for MyEnum {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}
// back_to_enum!(ai_sys::AIArtType);
//
// macro_rules! bla {
//     ($(#[$meta:meta])* $vis:vis enum $name:ident {
//         $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
//     }) => {
//         $(#[$meta])*
//         $vis enum $name {
//             $($(#[$vmeta])* $vname $(= $val)?,)*
//         }
// }
// }
//
// bla!(ai_sys::AIArtType);
