//procedure analyys (p : pchar; len : word); far stdcall external 'ana.dll';
//procedure tyybituvastus (p : pchar; len : word); far stdcall external 'ana.dll';
//procedure syntees (p : pchar; len : word); far stdcall external 'ana.dll';
//procedure sea_vxljundvorm (i : word); far stdcall external 'ana.dll';
//procedure sea_sqnastikuga (i : boolean); far stdcall external 'ana.dll';
//procedure sea_tuletusega (i : boolean); far stdcall external 'ana.dll';
//procedure sea_liitsqna (i : boolean); far stdcall external 'ana.dll';

extern crate encoding;
extern crate libloading as dll;

use std::ffi::CStr;
use std::io::{Error, ErrorKind};

use encoding::{DecoderTrap, EncoderTrap, Encoding};
use encoding::all::WINDOWS_1257;

static mut ANA_DLL: Option<dll::Library> = None;

fn analyys(s: &str) -> Result<String, String> {
    match WINDOWS_1257.encode(s, EncoderTrap::Strict) {
        Ok(vec) => analyze_encoded(vec),
        Err(e) => Err(e.into_owned())
    }
}

fn analyze_encoded(mut encoded_word: Vec<u8>) -> Result<String, String> {
    const LEN: u16 = 4096;
    encoded_word.resize(4096, 0);
    if let Err(e) = analyze_dll(encoded_word.as_mut_ptr() as *const u8, LEN) {
        return Err(e.to_string());
    }
    unsafe {
        let parsed_cstr = CStr::from_ptr(encoded_word.as_ptr() as *const i8);
        decode_analyze_result(parsed_cstr.to_bytes())
    }
}

fn decode_analyze_result(input: &[u8]) -> Result<String, String> {
    match WINDOWS_1257.decode(input, DecoderTrap::Strict) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.into_owned())
    }
}

fn analyze_dll(text: *const u8, len: u16) -> std::io::Result<()> {
    unsafe {
        match ANA_DLL {
            Some(ref lib) => {
                let external_analyze: libloading::Symbol<unsafe extern "stdcall" fn(p: *const u8, len: u16) -> ()> = lib.get(b"analyys")?;
                Ok(external_analyze(text, len))
            }
            None => Err(Error::from(ErrorKind::NotFound))
        }
    }
}

fn main() {
    unsafe {
        match dll::Library::new("ana.dll") {
            Ok(lib) => ANA_DLL = Some(lib),
            Err(_) => panic!("DLL not loaded")
        }
    }
    match analyys("tulla") {
        Result::Ok(s) => println!("Analyys for tulla: {:?}", s),
        Result::Err(e) => println!("Error occurred: {}", e)
    }
}
