//! Operators are used to indicate whether the parser has encountered
//! a standard operator or a custom one.

/// Standard set of operators + custom
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    /// Addition
    Addition,
    /// Subtraction **and** negation
    Subtraction,
    /// Multiplication
    Multiplication,
    /// Division
    Division,
    /// Modulus
    Modulus,
    /// Custom operator
    Custom
}