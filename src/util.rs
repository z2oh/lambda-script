/// Utility macro for quickly constructing a symbol table containing simple prelude definitions.
#[allow(unused_macros)]
macro_rules! prelude {
    () => (vec![(var_inner!("true"), t!()),
                (var_inner!("false"), f!()),
                (var_inner!("not"), not!()),
                (var_inner!("if"), if_!())].into_iter()
                                           .collect())
}

/// Utility macro for quickly constructing an application λ-term.
#[allow(unused_macros)]
macro_rules! app {
    ($e1:expr, $e2:expr) => (LambdaTerm::Application(Box::new(Application { term1: $e1, term2: $e2, })))
}

/// Utility macro for quickly constructing an abstraction λ-term.
#[allow(unused_macros)]
macro_rules! ab {
    ($e1:expr, $e2:expr) => (LambdaTerm::Abstraction(Box::new(Abstraction { bound: Variable { id: $e1.to_string(), }, body: $e2, })))
}

#[allow(unused_macros)]
macro_rules! var_inner {
    ($e:expr) => (Variable { id: $e.to_string(), })
}

/// Utility macro for quickly constructing a variable λ-term.
#[allow(unused_macros)]
macro_rules! var {
    ($e:expr) => (LambdaTerm::Variable(Box::new(var_inner!($e))))
}

/// The "true" λ-term.
#[allow(unused_macros)]
macro_rules! t {
    () => (
        ab!(
            "x",
            ab!(
                "y",
                var!("x")
            )
        )
    )
}

/// The "false" λ-term.
#[allow(unused_macros)]
macro_rules! f {
    () => (
        ab!(
            "x",
            ab!(
                "y",
                var!("y")
            )
        )
    )
}

/// The "if" λ-term.
#[allow(unused_macros)]
macro_rules! if_ {
    () => (
        ab!(
            "b",
            ab!(
                "t",
                ab!(
                    "f",
                    app!(
                        app!(
                            var!("b"),
                            var!("t")
                        ),
                        var!("f")
                    )
                )
            )
        )
    )
}

/// The "not" λ-term defined in terms of existing λ-terms (w/ substitution).
#[allow(unused_macros)]
macro_rules! not {
    () => (
        ab!(
            "b",
            app!(
                app!(
                    app!(
                        var!("if"),
                        var!("b")
                    ),
                    var!("false")
                ),
                var!("true")
            )
        )
    )
}

/// The "not" λ-term.
#[allow(unused_macros)]
macro_rules! not_full {
    () => (
        ab!(
            "b",
            app!(
                app!(
                    app!(
                        if_!(),
                        var!("b")
                    ),
                    f!()
                ),
                t!()
            )
        )
    )
}
