#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

use clap::App;

mod encoding;
mod analysis;
mod synthesis;
mod dynlib;

pub mod delphi_types {
    pub type Char = u8;
    pub type PChar = *const Char;
    pub type Integer = i32;
}

fn main() {

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // TODO: add support to specify analyze type code via command-line argument
    #[allow(unused_must_use)] {
        analysis::dll_set_analyze_type(1);
    }

    if let Some(submatch) = matches.subcommand_matches("analyze") {
        let word = submatch.value_of("WORD_TO_PROCESS").unwrap();
        match analysis::analyze(word) {
            Result::Ok(s) => print!("Analyze for {:?}:\n{}", word, s),
            Result::Err(e) => println!("Error occurred: {}", e)
        }
    }
    else if let Some(submatch) = matches.subcommand_matches("synthesize") {
        let word = submatch.value_of("WORD_TO_PROCESS").unwrap();
        match synthesis::synthesize(word) {
            Result::Ok(_) => print!("Synthesize for {:?}:\n", word),
            Result::Err(e) => println!("Error occurred: {}", e)
        }
    }
}
