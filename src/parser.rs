use crate::ast::{Abstraction, Application, LambdaTerm, Variable};
use nom::*;
use nom::types::CompleteStr;
use std::collections::HashMap;

pub struct Program {
    pub declarations: HashMap<Variable, LambdaTerm>,
    pub evaluation_term: LambdaTerm,
}

named!(_parse_program<CompleteStr, Program>,
    do_parse!(
        declarations: parse_declarations >>
        evaluation_term: lambda_term >>
        (Program {
            declarations,
            evaluation_term,
        })
    )
);

named!(parse_declarations<CompleteStr, HashMap<Variable, LambdaTerm>>,
    map!(many0!(
            ws!(do_parse!(
                var: variable >>
                tag!("=") >>
                term: lambda_term >>
                ((var, term))
            ))
        ),
        |v| v.into_iter().collect()
    )
);

named!(lambda_term<CompleteStr, LambdaTerm>,
    call!(application_term)
);

named!(application_term<CompleteStr, LambdaTerm>,
    do_parse!(
        term1: abstraction_term >>
        // Application is left associative. Ex. "not not true" would parse as "(not not) true".
        res: fold_many0!(
            preceded!(tag!(" "), abstraction_term),
            term1,
            |acc, term| {
                LambdaTerm::Application(Box::new(Application {
                    term1: acc,
                    term2: term,
                }))
            }
        ) >>
        (res)
    )
);

named!(abstraction_term<CompleteStr, LambdaTerm>,
    alt!(
        do_parse!(
            tag!("λ") >>
            bound: variable >>
            tag!(".") >>
            // Abstraction body parsing is greedy! The body will continue parsing until encountering
            // EOF or a closing parenthesis.
            body: lambda_term >>
            (LambdaTerm::Abstraction(Box::new(Abstraction { bound, body, })))
        ) |
        variable_term
    )
);

named!(variable<CompleteStr, Variable>,
    map!(call!(alpha), |id| Variable { id: id.to_string(), } )
);

named!(variable_term<CompleteStr, LambdaTerm>,
    alt!(
        map!(call!(variable), |var| LambdaTerm::Variable(Box::new(var))) |
        delimited!(tag!("("), lambda_term, tag!(")"))
    )
);

// TODO: Return Error instead of Option.
pub fn parse_program(input: &str) -> Option<Program> {
    let input = CompleteStr(input);
    match _parse_program(input) {
        Ok((rem, program)) => if rem.len() > 0 { None } else { Some(program) },
        _ => None,
    }
}

// TODO: Return Error instead of Option.
#[allow(dead_code)]
pub fn parse(input: &str) -> Option<LambdaTerm> {
    let input = CompleteStr(input);
    match lambda_term(input) {
        Ok((rem, term)) => if rem.len() > 0 { None } else { Some(term) },
        _ => None,
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_parser_parses() {
        assert!(parse("not not true").is_some())
    }

    #[test]
    fn test_parser_application_left_assoc() {
        let term = parse("not not true").unwrap();
        assert_eq!(term, app!(app!(var!("not"), var!("not")), var!("true")));
    }
}
