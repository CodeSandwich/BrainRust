extern crate brainrust;

use brainrust::{U8Editor, ExecutionError, ParseError, Program};
use std::io;
use std::io::Read;
use std::env;
use std::fs::File;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Expected 1 argument, source code file");
        return
    }
    let mut file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open file {}", &args[1]);
            return
        }
    };
    let mut source = String::new();
    if let Err(_) = file.read_to_string(&mut source) {
        println!("Failed to load source from file {}", &args[1]);
        return
    }
    let program = match Program::new(&mut source.chars()) {
        Ok(program) => program,
        Err(ParseError::LoopNeverClosed(index)) => {
            println!("File {}, character {}: unclosed loop", &args[1], index);
            return
        }
        Err(ParseError::NonexistentLoopClosed(index)) => {
            println!("File {}, character {}: closure of never opened loop", &args[1], index);
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
