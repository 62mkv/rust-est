use super::delphi_types as dt;
use super::dynlib;
use super::encoding;

const LEN: u16 = 4096;

type AnalyzeSymbol = unsafe extern "stdcall" fn(*const dt::Char, u16) -> ();
type FormCodeSymbol = unsafe extern "stdcall" fn(u16) -> ();

lazy_static! {
    static ref ANA_DLL: dynlib::Library = dynlib::initialize_dll("ana.dll");
    static ref ANA_FN: dynlib::Symbol<'static, AnalyzeSymbol> = dynlib::initialize_dll_function(&ANA_DLL, b"analyys");
    static ref FORM_CODE_FN: dynlib::Symbol<'static, FormCodeSymbol> = dynlib::initialize_dll_function(&ANA_DLL, b"sea_vxljundvorm");
}

fn process_encoded(mut encoded_word: Vec<u8>) -> Result<String, String> {
    encoded_word.resize(usize::from(LEN), 0);
    if let Err(e) = dll_analyze_word(encoded_word.as_mut_ptr() as *const dt::Char, LEN) {
        return Err(e.to_string());
    }
    encoding::decode(&encoded_word)
}

fn dll_analyze_word(text: *const dt::Char, len: u16) -> std::io::Result<()> {
    unsafe {
        Ok(ANA_FN(text, len))
    }
}

pub fn dll_set_analyze_type(code: u16) -> std::io::Result<()> {
    unsafe {
        Ok(FORM_CODE_FN(code))
    }
}

pub fn analyze(s: &str) -> Result<String, String> {
    match encoding::encode(s) {
        Ok(vec) => process_encoded(vec),
        Err(e) => Err(e.into_owned())
    }
}
