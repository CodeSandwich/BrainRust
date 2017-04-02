extern crate brainrust;
#[macro_use]
extern crate clap;

use brainrust::{U8Editor, ExecutionError, ParseError, Program};
use clap::{App, Arg};
use std::io;
use std::io::Read;
use std::fs::File;

pub fn main() {
    let matches = App::new("Brainrust CLI")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("source file")
            .required(true)
        )
        .get_matches();
    let filename_os_str = matches.value_of_os("source file").unwrap();
    let filename = matches.value_of_lossy("source file").unwrap();
    let mut file = match File::open(filename_os_str) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open file {}", &filename);
            return
        }
    };
    let mut source = String::new();
    if let Err(_) = file.read_to_string(&mut source) {
        println!("Failed to load source from file {}", filename);
        return
    }
    let program = match Program::new(&mut source.chars()) {
        Ok(program) => program,
        Err(ParseError::LoopNeverClosed(index)) => {
            println!("File {}, character {}: unclosed loop", filename, index);
            return
        }
        Err(ParseError::NonexistentLoopClosed(index)) => {
            println!("File {}, character {}: closure of never opened loop", filename, index);
            return
        }
    };
    let mut is_input_error = false;
    let execution_result = {
        let mut input = io::stdin().bytes()
            .take_while(|i| match *i {
                Err(_) => {
                    is_input_error = true;
                    false
                },
                _ => true,
            })
            .map(|i| i.unwrap());
        program.execute(&U8Editor::default(), &mut input, &mut io::stdout())
    };
    match execution_result {
        Err(ExecutionError::CannotReadFromInput) => {
            if is_input_error {
                println!("\nCould not read input")
            } else {
                println!("\nInput has ended")
            }
        },
        Err(ExecutionError::CannotWriteToOutput) => println!("\nCould not write to output"),
        Ok(()) => println!("\nProgram has ended"),
    }
}
