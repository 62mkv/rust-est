use std::ffi::CStr;

use lazy_static;

use super::delphi_types as dt;
use super::dynlib;
use super::encoding;

//    Function SynthesizeForms(lemma : PChar; withApp : integer; codeType : integer;
//     var outBuf : array of SynthFormSet; bufLength : integer) : integer; stdcall; external 'fmsynth.dll';
type SynthFn<'lib> = dynlib::Symbol<'lib, unsafe extern "stdcall" fn(dt::PChar, dt::Integer, dt::Integer, &mut [SynthFormSet], dt::Integer) -> dt::Integer>;

lazy_static! {
    static ref SYNTH_DLL: dynlib::Library = dynlib::initialize_dll("fmsynth.dll");
    static ref SYNTHESIZE_FN: SynthFn<'static> = dynlib::initialize_dll_function(&SYNTH_DLL, b"SynthesizeForms");
}

#[repr(C)]
#[repr(packed)]
struct SynthForm {
    //    vorm : array[0..29] of char;
    form: [u8; 30],
    //    stemLength : integer;
    stem_length: i32,
}

#[repr(C)]
#[repr(packed)]
struct SynthFormSet {
    //    tyybinumber : integer;
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

impl Default for SynthForm {
    fn default() -> Self {
        SynthForm {
            form: Default::default(),
            stem_length: 0,
        }
    }
}

impl Default for SynthFormSet {
    fn default() -> Self {
        SynthFormSet {
            type_number: 0,
            word_member: Default::default(),
            option_number: 0,
            parallel_forms: 0,
            form_code: Default::default(),
            forms: Default::default(),
        }
    }
}

pub fn synthesize(input: &str) -> Result<(), String> {
    let mut buffer: [SynthFormSet; 299] = array_init::array_init(|_| Default::default());
    let mut lemma = encoding::encode(input)?;
    lemma.resize(usize::from(30 as u8), 0);

    unsafe {
        let count = SYNTHESIZE_FN(lemma.as_ptr(), 0, 0, &mut buffer, 300);
        if count < 300 {
            for i in 0..count {
                let form_set = &buffer[i as usize];
                let type_number = form_set.type_number;
                let option_number = form_set.option_number;
                let parallel_forms = form_set.parallel_forms;
                let word_member = encoding::decode(&form_set.word_member)?;
                let form_code = encoding::decode(&form_set.form_code)?;
                println!("{}, {}, {}, {}, {}", word_member, type_number, option_number, parallel_forms, form_code);
                for j in 0..parallel_forms {
                    let form = encoding::decode(&form_set.forms[j as usize].form)?;
                    println!("{}", form);
                }
            }
        }
    }
    Ok(())
}
