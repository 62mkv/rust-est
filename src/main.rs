//procedure analyys (p : pchar; len : word); far stdcall external 'ana.dll';
//procedure tyybituvastus (p : pchar; len : word); far stdcall external 'ana.dll';
//procedure syntees (p : pchar; len : word); far stdcall external 'ana.dll';
//procedure sea_vxljundvorm (i : word); far stdcall external 'ana.dll';
//procedure sea_sqnastikuga (i : boolean); far stdcall external 'ana.dll';
//procedure sea_tuletusega (i : boolean); far stdcall external 'ana.dll';
//procedure sea_liitsqna (i : boolean); far stdcall external 'ana.dll';

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
    pub type Char = i8;
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
        let _word = submatch.value_of("WORD_TO_PROCESS").unwrap();
        println!("Synthesize is not implemented yet!")
    }
}
