#[macro_use]
extern crate clap;
#[macro_use]
extern crate enum_display_derive;
#[macro_use]
extern crate lazy_static;

use std::{env, fs};

use clap::App;

mod encoding;
mod analysis;
pub mod synthesis;
mod dynlib;
mod parser;

pub mod delphi_types {
    pub type Char = u8;
    pub type PChar = *const Char;
    pub type Integer = i32;
}

fn main() {
    let yaml = load_yaml!("cli-eki.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    println!("{:#?}", env::current_dir().unwrap().as_os_str());

    if let Some(submatch) = matches.subcommand_matches("process") {
        let filename = submatch.value_of("FILE_TO_PROCESS").unwrap();
        let folder = submatch.value_of("FOLDER_FOR_CSV").unwrap();

        println!("Parsing EKI XML file {}", filename);
        let contents = fs::read_to_string(filename)
            .expect("Unable to read a file");

        match parser::parse(&contents, &folder) {
            Ok(res) => println!("{}", res),
            Err(err) => eprintln!("{}", err)
        }
    }
}
