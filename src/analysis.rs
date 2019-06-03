extern crate libloading as dll;

use std::ffi::CStr;
use std::io::{Error, ErrorKind};

use super::delphi_types;
use super::encoding;

static mut ANA_DLL: Option<dll::Library> = None;
const LEN: u16 = 4096;

fn process_encoded(mut encoded_word: Vec<u8>, dll_function: &Fn(*const delphi_types::Char, u16) -> std::io::Result<()>) -> Result<String, String> {
    encoded_word.resize(usize::from(LEN), 0);
    if let Err(e) = dll_function(encoded_word.as_mut_ptr() as *const delphi_types::Char, LEN) {
        return Err(e.to_string());
    }
    unsafe {
        let parsed_cstr = CStr::from_ptr(encoded_word.as_ptr() as *const i8);
        encoding::decode(parsed_cstr.to_bytes())
    }
}

fn dll_analyze_word(text: *const delphi_types::Char, len: u16) -> std::io::Result<()> {
    unsafe {
        match ANA_DLL {
            Some(ref lib) => {
                let fun: dll::Symbol<unsafe extern "stdcall" fn(p: *const delphi_types::Char, len: u16) -> ()> = lib.get(b"analyys")?;
                Ok(fun(text, len))
            }
            None => Err(Error::from(ErrorKind::NotFound))
        }
    }
}

pub fn dll_set_analyze_type(code: u16) -> std::io::Result<()> {
    unsafe {
        match ANA_DLL {
            Some(ref lib) => {
                let fun: libloading::Symbol<unsafe extern "stdcall" fn(code: u16) -> ()> = lib.get(b"sea_vxljundvorm")?;
                Ok(fun(code))
            }
            None => Err(Error::from(ErrorKind::NotFound))
        }
    }
}

pub fn initialize() {
    unsafe {
        match dll::Library::new("ana.dll") {
            Ok(lib) => ANA_DLL = Some(lib),
            Err(_) => panic!("DLL not loaded")
        }
    }
}

pub fn analyze(s: &str) -> Result<String, String> {
    match encoding::encode(s) {
        Ok(vec) => process_encoded(vec, &dll_analyze_word),
        Err(e) => Err(e.into_owned())
    }
}
