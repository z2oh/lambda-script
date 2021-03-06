use std::fmt;

#[derive(Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum LambdaTerm {
    Variable(Box<Variable>),
    Application(Box<Application>),
    Abstraction(Box<Abstraction>),
}

impl fmt::Display for LambdaTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LambdaTerm::Variable(var_ref) => write!(f, "{}", *var_ref),
            LambdaTerm::Application(app_ref) => write!(f, "{}", *app_ref),
            LambdaTerm::Abstraction(ab_ref) => write!(f, "{}", *ab_ref),
        }
    }
}

impl fmt::Debug for LambdaTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LambdaTerm::Variable(var_ref) => write!(f, "{:#?}", *var_ref),
            LambdaTerm::Application(app_ref) => write!(f, "{:#?}", *app_ref),
            LambdaTerm::Abstraction(ab_ref) => write!(f, "{:#?}", *ab_ref),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// A variable defines a bound or free variable in an abstraction. Bound variables are replaced
/// during application. NOTE: `Variable` derives `Hash`, as it is used as the key for symbol table
/// entries.
pub struct Variable {
    pub id: String,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// An application is any term applied to any other term.
pub struct Application {
    pub term1: LambdaTerm,
    pub term2: LambdaTerm,
}

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.term1, &self.term2) {
            // If the first term is an abstraction, we must wrap it in parentheses to disambiguate
            // the second term from a continuation of the abstraction body. If the second term is
            // an application, we must wrap it in parentheses as application is normally
            // left-associative.
            (LambdaTerm::Abstraction(..), LambdaTerm::Application(..)) => write!(f, "({}) ({})", self.term1, self.term2),
            (LambdaTerm::Abstraction(..), _) => write!(f, "({}) {}", self.term1, self.term2),
            (_, LambdaTerm::Application(..)) => write!(f, "{} ({})", self.term1, self.term2),
            _ => write!(f, "{} {}", self.term1, self.term2),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Abstractions contain a single bound variable and a body. They take the form of `λbound.body`.
/// When an abstraction is on the left side of an application, the right side of the application is
/// put in the place of each instance of the bound variable in the abstraction body. The replaced
/// body is then returned as the evaluation of the application.
pub struct Abstraction {
    pub bound: Variable,
    pub body: LambdaTerm,
}

impl fmt::Display for Abstraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "λ{}.{}", self.bound, self.body)
    }
}
