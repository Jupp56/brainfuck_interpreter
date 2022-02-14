pub mod parser;
pub mod program;
pub mod interpreter;
use std::time::Instant;

use wasm_bindgen::prelude::*;


pub use crate::parser::*;
pub use crate::interpreter::*;



#[wasm_bindgen]
pub fn parse_and_run(input: String) -> String {
    let program = parse_program(input);

    match program {
        Ok(program) => {
            let start = Instant::now();
            match run_program(&program) {
                Ok(output) => {
                    let duration = start.elapsed();
                    println!("Execution took: {:?}", duration);
                    return output;
                },
                Err(e) => panic!("{:?}", e),
            }
           
            
        }
        Err(error) => match error {
            ParseError::BracketError(index) => {
                panic!(
                    "Parse error at index {}: no matching bracket found!",
                    index
                );
            }
        },
    }
}