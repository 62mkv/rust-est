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
#[derive(Copy, Clone)]
struct SynthForm {
    //    vorm : array[0..29] of char;
    form: [u8; 30],
    //    stemLength : integer;
    stem_length: i32,
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone)]
struct SynthFormSet {
    //    tyybinumber : integer;
    declination_type: i32,
    //    sqnaliik : array[0..2] of char;
    part_of_speech: [u8; 3],
    //    variandinumber : integer;
    number_of_options: i32,
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
            declination_type: 0,
            part_of_speech: Default::default(),
            number_of_options: 0,
            parallel_forms: 0,
            form_code: Default::default(),
            forms: Default::default(),
        }
    }
}

pub fn synthesize(input: &str) -> Result<(), String> {
    //    let mut buffer: [SynthFormSet; size] = array_init::array_init(|_| Default::default());
    const BUF_SIZE: i32 = 300;
    //let mut buffer: Box<[SynthFormSet; BUF_SIZE as usize]> = Box::new([Default::default(); BUF_SIZE as usize]);
    let mut buffer = Box::new([SynthFormSet::default(); (BUF_SIZE-1) as usize]);

    let mut lemma = encoding::encode(input)?;
    lemma.resize(usize::from(30 as u8), 0);

    unsafe {
        let count = SYNTHESIZE_FN(lemma.as_ptr(), 0, 0, &mut *buffer, BUF_SIZE);
        if count < BUF_SIZE {
            for i in 0..count {
                let form_set = &buffer[i as usize];
                let part_of_speech = encoding::decode(&form_set.part_of_speech)?;
                let form_code = encoding::decode(&form_set.form_code)?;
                let mut forms = "".to_string();
                for j in 0..form_set.parallel_forms {
                    let synth_form = &form_set.forms[j as usize];
                    if synth_form.stem_length > 0 {
                        let form = encoding::decode(&synth_form.form)?;
                        if j > 0 {
                            forms.push_str(" ~ ");
                        }
                        forms.push_str(&format!("{} ({})", form, synth_form.stem_length));
                    }
                }
                println!("{}, {}, {}, {}, {}, {}", part_of_speech, form_set.declination_type,
                         form_set.number_of_options, form_set.parallel_forms, form_code, forms);
            }
        }
    }
    Ok(())
}
