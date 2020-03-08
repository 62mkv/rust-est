use std::convert::{TryFrom, TryInto};

use lazy_static;

use super::delphi_types as dt;
use super::dynlib;
use super::encoding;

//    Function SynthesizeForms(lemma : PChar; withApp : integer; codeType : integer;
//     var outBuf : array of SynthFormSet; bufLength : integer) : integer; stdcall; external 'fmsynth.dll';
type SynthFn<'lib> = dynlib::Symbol<'lib, unsafe extern "stdcall" fn(dt::PChar, dt::Integer, dt::Integer, *mut SynthFormSet, dt::Integer) -> dt::Integer>;

lazy_static! {
    static ref SYNTH_DLL: dynlib::Library = dynlib::initialize_dll("fmsynth.dll");
    static ref SYNTHESIZE_FN: SynthFn<'static> = dynlib::initialize_dll_function(&SYNTH_DLL, b"SynthesizeForms");
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct SynthForm {
    //    vorm : array[0..29] of char;
    pub form: [u8; 30],
    //    stemLength : integer;
    pub stem_length: i32,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct SynthFormSet {
    //    tyybinumber : integer;
    pub declination_type: i32,
    //    sqnaliik : array[0..2] of char;
    pub part_of_speech: [u8; 3],
    //    variandinumber : integer;
    pub number_of_options: i32,
    //    paralleelvorme : integer;
    pub parallel_forms: i32,
    //    vormikood : array[0..29] of char;
    pub form_code: [u8; 30],
    //    vormid : array[0..4] of SynthForm;
    pub forms: [SynthForm; 5],
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
            declination_type: 0,
            part_of_speech: Default::default(),
            number_of_options: 0,
            parallel_forms: 0,
            form_code: Default::default(),
            forms: Default::default(),
        }
    }
}

const BUF_SIZE: usize = 300;

pub fn synthesize(input: &str) -> Result<(), String> {
    let (buffer, count) = synthesize_utf(input);

    let count = if count > BUF_SIZE { BUF_SIZE } else { count };

    for &SynthFormSet {
        declination_type,
        part_of_speech,
        number_of_options,
        parallel_forms,
        form_code,
        forms,
    } in &buffer[..count] {
        let part_of_speech = encoding::decode(&part_of_speech)?;
        let form_code = encoding::decode(&form_code)?;
        let mut forms_string = "".to_string();

        for &SynthForm {
            form,
            stem_length
        } in &forms[..usize::try_from(parallel_forms).expect("Overflow")] {
            if stem_length > 0 {
                let form_string = encoding::decode(&form)?;
                if forms_string.len() > 0 {
                    forms_string.push_str(" ~ ");
                }
                forms_string.push_str(&format!("{} ({})", form_string, stem_length));
            }
        }
        println!("{}, {}, {}, {}, {}, {}", part_of_speech, declination_type,
                 number_of_options, parallel_forms, form_code, forms_string);
    }

    Ok(())
}

fn synthesize_utf(input: &str) -> ([SynthFormSet; 300], usize) {
    let mut lemma = encoding::encode(input).unwrap();
    synthesize_encoded_vec(lemma)
}

pub fn synthesize_encoded_vec(mut lemma: Vec<u8>) -> ([SynthFormSet; 300], usize) {
    lemma.resize(usize::from(30 as u8), 0);
    let (buffer, count) = synthesize_encoded(lemma.as_ptr());
    (buffer, count)
}

fn synthesize_encoded(word: *const u8) -> ([SynthFormSet; BUF_SIZE], usize) {
    let mut buffer = [SynthFormSet::default(); BUF_SIZE];

    let count = usize::try_from(unsafe {
        SYNTHESIZE_FN(
            word,
            0,
            0,
            buffer.as_mut_ptr(),
            buffer.len().try_into().expect("Overflow"),
        )
    }).expect("Overflow");
    (buffer, count)
}