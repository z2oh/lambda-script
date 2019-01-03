use std::fmt;

#[derive(Clone)]
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

#[derive(Clone, Debug)]
/// A variable defines a bound or free variable in an abstraction. Bound variables are replaced
/// during application.
pub struct Variable {
    pub id: String,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Clone, Debug)]
/// An application is any term applied to any other term.
pub struct Application {
    pub term1: LambdaTerm,
    pub term2: LambdaTerm,
}

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.term1 {
            // If the first term is an abstraction, we must wrap it in parentheses to disambiguate
            // the second term from a continuation of the abstraction body.
            LambdaTerm::Abstraction(..) => write!(f, "({}) {}", self.term1, self.term2),
            _ => write!(f, "{} {}", self.term1, self.term2),
        }
    }
}

#[derive(Clone, Debug)]
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
