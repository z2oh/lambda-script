extern crate nom;

use std::io::{stdin, stdout, Write};

#[macro_use]
mod util;
mod ast;
mod eval;
mod parser;

use crate::eval::eval_step;
use crate::parser::parse_program;

fn main() {
    let ex = parse_program("true  = λx.λy.x
                            false = λx.λy.y

                            if = λb.λt.λf.b t f
                            not = if false true

                            not not true").unwrap();

    let mut term = ex.evaluation_term;

    let eval_context = eval::EvalContext { should_expand: true, symbol_table: ex.declarations };

    // Print out each step of evaluation until the user quits (with "q").
    loop {
        // Print out the current value.
        print!("{}", term);
        stdout().flush().unwrap();

        // Read in input.
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Quit on a "q" input.
        if input.starts_with('q') {
            break;
        }

        // Step forward in evaluation.
        term = eval_step(term, &eval_context);
    }
}
