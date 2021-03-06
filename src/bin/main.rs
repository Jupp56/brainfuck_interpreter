extern crate brainfuck_interpreter;
use self::brainfuck_interpreter::*;

use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::env::args;

fn main() {
    let path = args().nth(1).expect("Error: please provide a path of the file to interpret.");
    let path = OsStr::new(&path);
    let mut file_content = read_file(path);
    let program = parse_program(&mut file_content);

    match program {
        Ok(program) => {
            let start = Instant::now();
            match run_program(&program) {
                Ok(_) => (),
                Err(e) => panic!(e),
            }
            let duration = start.elapsed();
            println!("Execution took: {:?}", duration);
        }
        Err(error) => match error {
            ParseError::BracketError(index) => {
                panic!(format!(
                    "Parse error at index {}: no matching bracket found!",
                    index
                ));
            }
        },
    }
}

fn read_file(path: &OsStr) -> String {
    let mut file: File =
        File::open(path).expect(format!("Could not open file \"{:?}\"!", path).as_str());
    let mut contents: String = String::new(); //where our text read from file will live
    file.read_to_string(&mut contents)
        .expect("Error while reading file!");
    contents
}
