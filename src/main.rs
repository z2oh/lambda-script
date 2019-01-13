extern crate nom;

use std::io::{stdin, stdout, Write};

#[macro_use]
mod util;
mod ast;
mod eval;
mod parser;

use crate::eval::eval_step;
use crate::parser::{parse_script, Script};

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let script = read_script_file("./examples/add.λ")?;

    match script {
        Script::Library { .. } => {
            println!("The supplied file is a library.");
            Ok(())
        },
        Script::Program { declarations, mut evaluation_term, } => {
            let eval_context = eval::EvalContext {
                should_expand: true,
                symbol_table: declarations,
            };

            // Print out each step of evaluation until the user quits (with "q").
            loop {
                // Print out the current value.
                print!("{}", evaluation_term);
                stdout().flush().unwrap();

                // Read in input.
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();

                // Quit on a "q" input.
                if input.starts_with('q') {
                    break Ok(());
                }

                // Step forward in evaluation.
                evaluation_term = eval_step(evaluation_term, &eval_context);
            }
        },
    }
}

fn read_script_file(path: &str) -> Result<Script, Box<Error>> {
    let mut file = File::open(path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    Ok(parse_script(&file_contents)?)
}
