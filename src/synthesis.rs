use lazy_static;

use super::delphi_types as dt;
use super::dynlib;

//    Function SynthesizeForms(lemma : PChar; withApp : integer; codeType : integer;
//     var outBuf : array of SynthFormSet; bufLength : integer) : integer; stdcall; external 'fmsynth.dll';
type SynthFn<'lib> = dynlib::Symbol<'lib, unsafe extern "stdcall" fn(dt::PChar, dt::Integer, dt::Integer, &mut [SynthFormSet], dt::Integer) -> dt::Integer>;

lazy_static! {
    static ref SYNTH_DLL: dynlib::Library = dynlib::initialize_dll("fmsynth.dll");
    static ref SYNTHESIZE_FN: SynthFn<'static> = dynlib::initialize_dll_function(&SYNTH_DLL, b"SynthesizeForms");
}

#[allow(dead_code)]
struct SynthForm {
    //    vorm : array[0..29] of char;
    form: [u8; 30],
    //    stemLength : integer;
    stem_length: i32,
}

#[allow(dead_code)]
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
