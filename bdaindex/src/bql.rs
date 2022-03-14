mod parser;
mod scanner;
use serde::{
    self,
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_json::Value as JValue;
use std::{
    cmp::Ordering,
    fmt::{self, Display},
    hash::Hash,
    num::ParseFloatError,
    str::FromStr,
};

pub fn from_str(s: &str) -> Result<BQL, String> {
    parser::parse(s)
}

#[derive(PartialEq, Debug, Clone, PartialOrd, Eq, Serialize, Deserialize, Hash)]
pub enum Value {
    Bottom,
    Rational(Rational),
    Integral(i64),
    Text(String),
    Boolean(bool),
    Top,
}

#[derive(PartialEq, Debug, Clone)]
pub enum BQL {
    And(Box<BQL>, Box<BQL>),
    Or(Box<BQL>, Box<BQL>),
    Diff(Box<BQL>, Box<BQL>),
    Comp(Box<BQL>, Box<BQL>),
    Not(Box<BQL>),
    IsPresent,
    Eq { field: String, value: Value },
    IsDefined { field: String },
    LT { field: String, value: Value },
    LE { field: String, value: Value },
    GT { field: String, value: Value },
    GE { field: String, value: Value },
    All { field: String, values: Vec<Value> },
    Any { field: String, values: Vec<Value> },
}

impl Value {
    pub fn from_json(json: JValue) -> Self {
        match json {
            JValue::Bool(vv) => Value::Boolean(vv),
            JValue::Number(vv) => match vv.as_f64() {
                Some(n) => Value::Rational(Rational::from(n)),
                None => Value::Rational(Rational::from(f64::NAN)),
            },
            JValue::String(vv) => Value::Text(vv),
            _ => Value::Bottom,
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Value::Text(t) => match other {
                Value::Text(tt) => t.cmp(tt),
                Value::Top => Ordering::Less,
                Value::Boolean(_) => Ordering::Less,
                Value::Rational(_) => Ordering::Greater,
                Value::Integral(_) => Ordering::Greater,
                Value::Bottom => Ordering::Greater,
            },
            Value::Boolean(b) => match other {
                Value::Boolean(bb) => b.cmp(bb),
                Value::Top => Ordering::Less,
                Value::Text(_) => Ordering::Greater,
                Value::Rational(_) => Ordering::Greater,
                Value::Integral(_) => Ordering::Greater,
                Value::Bottom => Ordering::Greater,
            },
            Value::Integral(n) => match other {
                Value::Integral(nn) => n.cmp(nn),
                Value::Rational(nn) if nn.value.is_nan() => Ordering::Greater,
                Value::Rational(nn) if (*n as f64) > nn.value => Ordering::Greater,
                Value::Rational(nn) if (*n as f64) < nn.value => Ordering::Less,
                Value::Rational(_) => Ordering::Equal,
                Value::Top => Ordering::Less,
                Value::Text(_) => Ordering::Less,
                Value::Boolean(_) => Ordering::Less,
                Value::Bottom => Ordering::Greater,
            },
            Value::Rational(n) if n.value.is_nan() => match other {
                Value::Rational(nn) if nn.value.is_nan() => Ordering::Equal,
                Value::Bottom => Ordering::Greater,
                Value::Top => Ordering::Less,
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
                Value::Top => Ordering::Less,
                Value::Text(_) => Ordering::Less,
                Value::Boolean(_) => Ordering::Less,
                Value::Bottom => Ordering::Greater,
            },
            Value::Bottom => match other {
                &Value::Bottom => Ordering::Equal,
                _ => Ordering::Less,
            },
            Value::Top => match other {
                &Value::Top => Ordering::Equal,
                _ => Ordering::Greater,
            },
        }
    }
}

#[derive(Debug, Clone, PartialOrd)]
pub struct Rational {
    pub value: f64,
}

impl Hash for Rational {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.to_bits().hash(state);
    }
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
