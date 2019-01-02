use std::io::{stdin, stdout, Write};

#[macro_use]
mod util;
mod ast;
mod eval;

use crate::ast::*;
use crate::eval::eval_step;

fn main() {
    // Using the helper macros, build a small example "not not false".
    let mut ex = app!(var!("not"), app!(var!("not"), var!("false")));

    let eval_context = eval::EvalContext { should_expand: true, symbol_table: prelude!() };

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
        ex = eval_step(ex, &eval_context);
    }
}
