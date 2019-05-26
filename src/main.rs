//procedure analyys (p : pchar; len : word); far stdcall external 'ana.dll';
//procedure tyybituvastus (p : pchar; len : word); far stdcall external 'ana.dll';
//procedure syntees (p : pchar; len : word); far stdcall external 'ana.dll';
//procedure sea_vxljundvorm (i : word); far stdcall external 'ana.dll';
//procedure sea_sqnastikuga (i : boolean); far stdcall external 'ana.dll';
//procedure sea_tuletusega (i : boolean); far stdcall external 'ana.dll';
//procedure sea_liitsqna (i : boolean); far stdcall external 'ana.dll';

#[macro_use]
extern crate clap;
extern crate encoding;
extern crate libloading as dll;

use std::ffi::CStr;
use std::io::{Error, ErrorKind};

use clap::App;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use encoding::all::WINDOWS_1257;

static mut ANA_DLL: Option<dll::Library> = None;

fn analyys(s: &str) -> Result<String, String> {
    match WINDOWS_1257.encode(s, EncoderTrap::Strict) {
        Ok(vec) => process_encoded(vec, &dll_analyze_word),
        Err(e) => Err(e.into_owned())
    }
}

fn decode_result(input: &[u8]) -> Result<String, String> {
    match WINDOWS_1257.decode(input, DecoderTrap::Strict) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.into_owned())
    }
}

fn process_encoded(mut encoded_word: Vec<u8>, dll_function: &Fn(*const u8, u16) -> std::io::Result<()>) -> Result<String, String> {
    const LEN: u16 = 4096;
    encoded_word.resize(4096, 0);
    if let Err(e) = dll_function(encoded_word.as_mut_ptr() as *const u8, LEN) {
        return Err(e.to_string());
    }
    unsafe {
        let parsed_cstr = CStr::from_ptr(encoded_word.as_ptr() as *const i8);
        decode_result(parsed_cstr.to_bytes())
    }
}

fn dll_analyze_word(text: *const u8, len: u16) -> std::io::Result<()> {
    unsafe {
        match ANA_DLL {
            Some(ref lib) => {
                let fun: libloading::Symbol<unsafe extern "stdcall" fn(p: *const u8, len: u16) -> ()> = lib.get(b"analyys")?;
                Ok(fun(text, len))
            }
            None => Err(Error::from(ErrorKind::NotFound))
        }
    }
}

fn dll_set_analyze_type(code: u16) -> std::io::Result<()> {
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

fn main() {
    unsafe {
        match dll::Library::new("ana.dll") {
            Ok(lib) => ANA_DLL = Some(lib),
            Err(_) => panic!("DLL not loaded")
        }
    }

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // TODO: add support to specify analyze type code via command-line argument
    #[allow(unused_must_use)] {
        dll_set_analyze_type(1);
    }

    if let Some(submatch) = matches.subcommand_matches("analyze") {
        let word = submatch.value_of("WORD_TO_PROCESS").unwrap();
        match analyys(word) {
            Result::Ok(s) => print!("Analyze for {:?}:\n{}", word, s),
            Result::Err(e) => println!("Error occurred: {}", e)
        }
    }
    else if let Some(submatch) = matches.subcommand_matches("synthesize") {
        let _word = submatch.value_of("WORD_TO_PROCESS").unwrap();
        println!("Synthesize is not implemented yet!")
    }
}
