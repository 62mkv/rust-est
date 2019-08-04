extern crate encoding;

use std::borrow::Cow;
use std::ffi::CStr;

use encoding::{DecoderTrap, EncoderTrap, Encoding};
use encoding::all::WINDOWS_1257;

pub fn decode(input: &[u8]) -> Result<String, String> {
    unsafe {
        let parsed_cstr = CStr::from_ptr(input.as_ptr() as *const i8);
        match WINDOWS_1257.decode(parsed_cstr.to_bytes(), DecoderTrap::Strict) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into_owned())
        }
    }
}

pub fn encode(input: &str) -> Result<Vec<u8>, Cow<'static, str>> {
    WINDOWS_1257.encode(input, EncoderTrap::Strict)
}