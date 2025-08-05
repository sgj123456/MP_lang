use std::{
    collections::HashMap,
    fmt::{self, Display},
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

use crate::runtime::environment::function::Function;

#[derive(Debug, Clone)]
pub enum EnvironmentValue {
    Variable(Value),
    Function(Function),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(i128),
    Float(f64),
}

impl Number {
    pub fn to_int(&self) -> i128 {
        match self {
            Number::Int(i) => *i,
            Number::Float(f) => *f as i128,
        }
    }
    pub fn to_float(&self) -> f64 {
        match self {
            Number::Int(i) => *i as f64,
            Number::Float(f) => *f,
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(i) => write!(f, "{i}"),
            Number::Float(fl) => write!(f, "{fl:?}"),
        }
    }
}

impl From<Number> for i128 {
    fn from(n: Number) -> Self {
        match n {
            Number::Int(i) => i,
            _ => panic!("Cannot convert non-integer number to i128"),
        }
    }
}

impl From<Number> for f64 {
    fn from(n: Number) -> Self {
        match n {
            Number::Float(f) => f,
            _ => panic!("Cannot convert non-float number to f64"),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Number::Int(i1), Number::Int(i2)) => Number::Int(i1 + i2),
            (Number::Float(f1), Number::Float(f2)) => Number::Float(f1 + f2),
            _ => panic!("Cannot add non-numeric values"),
        }
    }
}
impl Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Number::Int(i1), Number::Int(i2)) => Number::Int(i1 - i2),
            (Number::Float(f1), Number::Float(f2)) => Number::Float(f1 - f2),
            _ => panic!("Cannot subtract non-numeric values"),
        }
    }
}
impl Mul for Number {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Number::Int(i1), Number::Int(i2)) => Number::Int(i1 * i2),
            (Number::Float(f1), Number::Float(f2)) => Number::Float(f1 * f2),
            _ => panic!("Cannot multiply non-numeric values"),
        }
    }
}
impl Div for Number {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Number::Int(i1), Number::Int(i2)) => Number::Int(i1 / i2),
            (Number::Float(f1), Number::Float(f2)) => Number::Float(f1 / f2),
            _ => panic!("Cannot divide non-numeric values"),
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Number::Int(i1), Number::Int(i2)) => i1.partial_cmp(i2),
            (Number::Float(f1), Number::Float(f2)) => f1.partial_cmp(f2),
            _ => None,
        }
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Number::Int(i) => Number::Int(-i),
            Number::Float(f) => Number::Float(-f),
        }
    }
}

impl From<i128> for Number {
    fn from(i: i128) -> Self {
        Number::Int(i)
    }
}

impl From<f64> for Number {
    fn from(f: f64) -> Self {
        Number::Float(f)
    }
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i128>() {
            Ok(i) => Ok(Number::Int(i)),
            Err(_) => match s.parse::<f64>() {
                Ok(f) => Ok(Number::Float(f)),
                Err(_) => Err(()),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(Number),
    Boolean(bool),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => match n {
                Number::Int(i) => write!(f, "{i}"),
                Number::Float(fl) => write!(f, "{fl:?}"),
            },
            Value::Boolean(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Array(v) => {
                write!(f, "[")?;
                for (i, item) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, "]")
            }
            Value::Object(o) => {
                write!(f, "{{")?;
                for (i, (k, v)) in o.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{k}: {v}")?;
                }
                write!(f, "}}")
            }
            Value::Nil => write!(f, "nil"),
        }
    }
}
