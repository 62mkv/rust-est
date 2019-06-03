extern crate encoding;

use std::borrow::Cow;

use encoding::{DecoderTrap, EncoderTrap, Encoding};
use encoding::all::WINDOWS_1257;

pub fn decode(input: &[u8]) -> Result<String, String> {
    match WINDOWS_1257.decode(input, DecoderTrap::Strict) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.into_owned())
    }
}

pub fn encode(input: &str) -> Result<Vec<u8>, Cow<'static, str>> {
    WINDOWS_1257.encode(input, EncoderTrap::Strict)
}