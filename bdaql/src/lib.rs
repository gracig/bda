mod parser;
mod scanner;
use serde::{
    self,
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
    num::ParseFloatError,
    str::FromStr,
};

pub fn from_str(s: &str) -> Result<Ast, String> {
    parser::parse(s).map_err(|e| format!("error: {} parsing bql: {}", e.to_string(), s))
}

#[derive(PartialEq, Debug, Clone, PartialOrd, Eq, Serialize, Deserialize)]
pub enum Value {
    Rational(Rational),
    Integral(i64),
    Text(String),
    Boolean(bool),
}
#[derive(PartialEq, Debug, Clone)]
pub enum Ast {
    Intersection(Box<Ast>, Box<Ast>),
    Union(Box<Ast>, Box<Ast>),
    Difference(Box<Ast>, Box<Ast>),
    Complement(Box<Ast>, Box<Ast>),
    All,
    Equal {
        fname: String,
        fvalue: Option<Value>,
        negate: bool,
    },
    Defined {
        fname: String,
        negate: bool,
    },
    LessThan {
        fname: String,
        fvalue: Option<Value>,
    },
    LessThanOrEqual {
        fname: String,
        fvalue: Option<Value>,
    },
    GreaterThan {
        fname: String,
        fvalue: Option<Value>,
    },
    GreaterThanOrEqual {
        fname: String,
        fvalue: Option<Value>,
    },
    ContainsAll {
        fname: String,
        fvalues: Vec<Option<Value>>,
        negate: bool,
    },
    ContainsAny {
        fname: String,
        fvalues: Vec<Option<Value>>,
        negate: bool,
    },
}

#[derive(Debug, Clone, PartialOrd)]
pub struct Rational {
    pub value: f64,
}
impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        if self.value.is_nan() && other.value.is_nan() {
            true
        } else {
            self.value == other.value
        }
    }
}
impl Eq for Rational {
    fn assert_receiver_is_total_eq(&self) {}
}
impl FromStr for Rational {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rational::from(f64::from_str(s)?))
    }
}
impl Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<f64> for Rational {
    fn from(n: f64) -> Self {
        Rational { value: n }
    }
}
impl From<f32> for Rational {
    fn from(n: f32) -> Self {
        Rational {
            value: f64::from(n),
        }
    }
}
impl From<i8> for Rational {
    fn from(n: i8) -> Self {
        Rational {
            value: f64::from(n),
        }
    }
}
impl From<i32> for Rational {
    fn from(n: i32) -> Self {
        Rational {
            value: f64::from(n),
        }
    }
}
impl From<u8> for Rational {
    fn from(n: u8) -> Self {
        Rational {
            value: f64::from(n),
        }
    }
}
impl From<u32> for Rational {
    fn from(n: u32) -> Self {
        Rational {
            value: f64::from(n),
        }
    }
}

impl Into<f64> for Rational {
    fn into(self) -> f64 {
        self.value
    }
}

impl Into<f64> for &Rational {
    fn into(self) -> f64 {
        self.value
    }
}

impl Serialize for Rational {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f64(self.value)
    }
}

impl<'de> Deserialize<'de> for Rational {
    fn deserialize<D>(deserializer: D) -> Result<Rational, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_f64(NumberVisitor)
    }
}
struct NumberVisitor;

impl<'de> Visitor<'de> for NumberVisitor {
    type Value = Rational;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }
    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Rational::from(value))
    }
    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Rational::from(value))
    }
    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Rational::from(value))
    }
    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Rational::from(value))
    }
    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Rational::from(value))
    }
    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Rational::from(value))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Value::Text(t) => match other {
                Value::Text(tt) => t.cmp(tt),
                Value::Rational(_) => Ordering::Greater,
                Value::Boolean(_) => Ordering::Less,
                Value::Integral(_) => Ordering::Greater,
            },
            Value::Boolean(b) => match other {
                Value::Boolean(bb) => b.cmp(bb),
                Value::Rational(_) => Ordering::Greater,
                Value::Text(_) => Ordering::Greater,
                Value::Integral(_) => Ordering::Greater,
            },
            Value::Integral(n) => match other {
                Value::Integral(nn) => n.cmp(nn),
                Value::Rational(nn) if nn.value.is_nan() => Ordering::Greater,
                Value::Rational(nn) if (*n as f64) > nn.value => Ordering::Greater,
                Value::Rational(nn) if (*n as f64) < nn.value => Ordering::Less,
                Value::Rational(_) => Ordering::Equal,
                Value::Text(_) => Ordering::Less,
                Value::Boolean(_) => Ordering::Less,
            },
            Value::Rational(n) if n.value.is_nan() => match other {
                Value::Rational(nn) if nn.value.is_nan() => Ordering::Equal,
                _ => Ordering::Less,
            },
            Value::Rational(n) => match other {
                Value::Rational(nn) if nn.value.is_nan() => Ordering::Greater,
                Value::Rational(nn) if n > nn => Ordering::Greater,
                Value::Rational(nn) if n < nn => Ordering::Less,
                Value::Rational(_) => Ordering::Equal,
                Value::Integral(nn) if n.value > (*nn as f64) => Ordering::Greater,
                Value::Integral(nn) if n.value < (*nn as f64) => Ordering::Less,
                Value::Integral(_) => Ordering::Equal,
                Value::Text(_) => Ordering::Less,
                Value::Boolean(_) => Ordering::Less,
            },
        }
    }
}
