use crate::ast::{Abstraction, Application, LambdaTerm, Variable};
use nom::types::CompleteStr;
use nom::*;
use std::collections::HashMap;
use std::fmt;
use std::error::Error;

pub enum Script {
    Library {
        declarations: HashMap<Variable, LambdaTerm>,
    },
    Program {
        declarations: HashMap<Variable, LambdaTerm>,
        evaluation_term: LambdaTerm,
    },
}

#[derive(Debug)]
pub enum ParseError {
    RemainingInput(String),
    InvalidInput,
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::RemainingInput(rem) => {
                write!(f, "Finished parsing, but experienced more input:\n```\n{})\n```", rem)
            },
            ParseError::InvalidInput => {
                write!(f, "The input was invalid!")
            }
        }
    }
}

named!(_parse_script<CompleteStr, Script>,
    do_parse!(
        declarations: parse_declarations >>
        evaluation_term: opt!(lambda_term) >>
        ws!(eof!()) >>
        (match evaluation_term {
            Some(evaluation_term) => Script::Program {
                declarations,
                evaluation_term,
            },
            None => Script::Library {
                declarations,
            },
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
    map!(is_not!("\t\n λ.=()"), |id| Variable { id: id.to_string(), } )
);

named!(variable_term<CompleteStr, LambdaTerm>,
    alt!(
        map!(call!(variable), |var| LambdaTerm::Variable(Box::new(var))) |
        delimited!(tag!("("), lambda_term, tag!(")"))
    )
);

pub fn parse_script(input: &str) -> Result<Script, ParseError> {
    let input = CompleteStr(input);
    match _parse_script(input) {
        Ok((rem, script)) => {
            if rem.len() > 0 {
                Err(ParseError::RemainingInput(rem.to_string()))
            } else {
                Ok(script)
            }
        }
        Err(_) => Err(ParseError::InvalidInput),
    }
}

#[allow(dead_code)]
pub fn parse(input: &str) -> Result<LambdaTerm, ParseError> {
    let input = CompleteStr(input);
    match lambda_term(input) {
        Ok((rem, term)) => {
            if rem.len() > 0 {
                Err(ParseError::RemainingInput(rem.to_string()))
            } else {
                Ok(term)
            }
        }
        Err(_) => Err(ParseError::InvalidInput),
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
