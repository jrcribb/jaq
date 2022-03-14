use core::fmt;
use core::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, Debug, PartialEq)]
pub enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl MathOp {
    pub fn run<I, O>(&self, l: I, r: I) -> O
    where
        I: Add<Output = O>,
        I: Sub<Output = O>,
        I: Mul<Output = O>,
        I: Div<Output = O>,
        I: Rem<Output = O>,
    {
        match self {
            Self::Add => l + r,
            Self::Sub => l - r,
            Self::Mul => l * r,
            Self::Div => l / r,
            Self::Rem => l % r,
        }
    }
}

impl fmt::Display for MathOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add => "+".fmt(f),
            Self::Sub => "-".fmt(f),
            Self::Mul => "*".fmt(f),
            Self::Div => "/".fmt(f),
            Self::Rem => "%".fmt(f),
        }
    }
}

/// An operation that takes two values and returns a boolean value.
#[derive(Clone, Debug)]
pub enum OrdOp {
    /// Less-than (<).
    Lt,
    /// Less-than or equal (<=).
    Le,
    /// Greater-than (>).
    Gt,
    /// Greater-than or equal (>=).
    Ge,
    /// Equals (=).
    Eq,
    /// Not equals (!=).
    Ne,
}

impl OrdOp {
    pub fn run<I: PartialOrd + PartialEq>(&self, l: &I, r: &I) -> bool {
        match self {
            Self::Gt => l > r,
            Self::Ge => l >= r,
            Self::Lt => l < r,
            Self::Le => l <= r,
            Self::Eq => l == r,
            Self::Ne => l != r,
        }
    }
}

impl fmt::Display for OrdOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lt => "<".fmt(f),
            Self::Gt => ">".fmt(f),
            Self::Le => "<=".fmt(f),
            Self::Ge => ">=".fmt(f),
            Self::Eq => "==".fmt(f),
            Self::Ne => "!=".fmt(f),
        }
    }
}
