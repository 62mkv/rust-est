extern crate libloading as dll;

use std::io::{Error, ErrorKind};

use lazy_static;

use super::delphi_types as dt;
use super::encoding;

pub type SynthFn<'lib> = dll::Symbol<'lib, unsafe extern "stdcall" fn(dt::PChar, dt::Integer, dt::Integer, &mut [SynthFormSet], dt::Integer) -> dt::Integer>;

lazy_static! {
    static ref SYNTH_DLL: dll::Library = initialize_dll();
    static ref SYNTHESIZE_FN: SynthFn<'static> = initialize_dll_fn();
}

struct SynthForm {
    //    vorm : array[0..29] of char;
    form: [u8; 30],
    //    stemLength : integer;
    stem_length: i32,
}

struct SynthFormSet {
    type_number: i32,
    //    sqnaliik : array[0..2] of char;
    word_member: [u8; 3],
    //    variandinumber : integer;
    option_number: i32,
    //    paralleelvorme : integer;
    parallel_forms: i32,
    //    vormikood : array[0..29] of char;
    form_code: [u8; 30],
    //    vormid : array[0..4] of SynthForm;
    forms: [SynthForm; 5],
}

//    Function SynthesizeForms
//        (lemma : PChar; withApp : integer; codeType : integer;
//    var outBuf : array of SynthFormSet; bufLength : integer) : integer;
//    stdcall; external 'fmsynth.dll';
fn initialize_dll() -> dll::Library {
    match dll::Library::new("fmsynth.dll") {
        Ok(lib) => lib,
        Err(_) => panic!("DLL not loaded")
    }
}

fn initialize_dll_fn() -> SynthFn<'static> {
    unsafe {
        match SYNTH_DLL.get(b"SynthesizeForms") {
            Ok(fun) => {
                fun
//            let x: &'static SynthFn = &fun;
//            x
            }
            Err(_) => panic!("Function not found!")
        }
    }
}

