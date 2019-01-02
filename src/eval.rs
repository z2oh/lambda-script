use std::collections::HashMap;

use crate::ast::*;

pub struct EvalContext {
    pub should_expand: bool,
    /// The symbol table contains function definitions. Free variables are looked up in this
    /// table for replacement if no more evaluation can be performed. We call this process
    /// "expansion".
    pub symbol_table: HashMap<String, LambdaTerm>,
}

/// Moves evaluation forward by a single step.
pub fn eval_step(term: LambdaTerm, context: &EvalContext) -> LambdaTerm {
    match term {
        // Perform the application.
        LambdaTerm::Application(app_ref) => eval_application(Application {
                                                // We recurse only on the left side of an
                                                // application.
                                                term1: eval_step(app_ref.term1, context),
                                                term2: app_ref.term2,
                                            }),
        // Expand a free variable.
        // TODO: Test that bound variables are not expanded.
        LambdaTerm::Variable(ref var_ref) if context.should_expand &&
                                            context
                                            .symbol_table
                                            .contains_key(&var_ref.id) => context
                                                                         .symbol_table
                                                                         .get(&var_ref.id)
                                                                         .unwrap()
                                                                         .clone(),
        term => term,
    }
}

/// If the first term is an abstraction, then the application is performed by replacing the bound
/// variable with the second term. If the first term is not an abstraction, then the original
/// application is returned.
fn eval_application(Application { term1, term2 }: Application) -> LambdaTerm {
    match term1 {
        LambdaTerm::Abstraction(ab_ref) => replace_bound(ab_ref.body, &ab_ref.bound, &term2),
        _ => LambdaTerm::Application(Box::new(Application {
            term1,
            term2,
        })),
    }
}

/// Replaces a bound variable `bound` in a λ-term `term` with the value of another λ-term,
/// `replace`. This is where the actual "application" happens.
fn replace_bound(term: LambdaTerm, bound: &Variable, replace: &LambdaTerm) -> LambdaTerm {
    match term.clone() {
        // If the variable id matches the bound variable id, we replace it.
        LambdaTerm::Variable(var_ref) => {
            if var_ref.id == bound.id {
                replace.clone()
            } else {
                term
            }
        }
        // We recurse into both sides of the application.
        LambdaTerm::Application(app_ref) => {
            LambdaTerm::Application(Box::new(Application {
                term1: replace_bound(app_ref.term1, bound, replace),
                term2: replace_bound(app_ref.term2, bound, replace),
            }))
        }
        // As long as we do not bind the same id, we recurse into the body of the abstraction to
        // continue replacement.
        LambdaTerm::Abstraction(ab_ref) => {
            // TODO: Is this check necessary?
            if ab_ref.bound.id != bound.id {
                LambdaTerm::Abstraction(Box::new(Abstraction {
                    bound: ab_ref.bound.clone(),
                    body: replace_bound(ab_ref.body, bound, replace),
                }))
            } else {
                term
            }
        }
    }
}
