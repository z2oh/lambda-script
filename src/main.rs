use std::io::{stdin, stdout, Write};

#[macro_use]
mod util;
mod ast;
mod eval;

use crate::ast::*;
use crate::eval::eval_step;

fn main() {
    // Using the helper macros, build a small example (NOT FALSE).
    let mut ex = app!(not!(), f!());

    // Print out each step of evaluation until the user quits (with "q").
    loop {
        // Print out the current value.
        print!("{}", ex);
        stdout().flush().unwrap();

        // Read in input.
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Quit on a "q" input.
        if input.starts_with('q') {
            break;
        }

        // Step forward in evaluation.
        ex = eval_step(ex);
    }
}
