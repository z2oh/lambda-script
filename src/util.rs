/// Utility macro for quickly constructing an application λ-term.
macro_rules! app {
    ($e1:expr, $e2:expr) => (LambdaTerm::Application(Box::new(Application { term1: $e1, term2: $e2, })))
}

/// Utility macro for quickly constructing an abstraction λ-term.
macro_rules! ab {
    ($e1:expr, $e2:expr) => (LambdaTerm::Abstraction(Box::new(Abstraction { bound: Variable { id: $e1.to_string(), }, body: $e2, })))
}

/// Utility macro for quickly constructing a variable λ-term.
macro_rules! var {
    ($e:expr) => (LambdaTerm::Variable(Box::new(Variable { id: $e.to_string(), })))
}

/// The "true" λ-term.
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
macro_rules! if_ {
    () => (
        ab!(
            "b1",
            ab!(
                "t",
                ab!(
                    "f",
                    app!(
                        app!(
                            var!("b1"),
                            var!("t")
                        ),
                        var!("f")
                    )
                )
            )
        )
    )
}

/// The "not" λ-term.
macro_rules! not {
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
