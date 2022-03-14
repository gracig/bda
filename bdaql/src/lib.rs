use std::cmp::Ordering;

mod parser;
mod scanner;

pub fn from_str(s: &str) -> Result<Ast, String> {
    return parser::parse(s);
}

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum Value {
    Number(f64),
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

//To be used as key index. Should implement Ord. f64 does not implement Ord
//So a naive implementation was given below
impl Eq for Value {
    fn assert_receiver_is_total_eq(&self) {}
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Value::Number(n) => match other {
                Value::Number(nn) => {
                    if n.is_nan() {
                        if nn.is_nan() {
                            Ordering::Equal
                        } else {
                            Ordering::Less
                        }
                    } else if nn.is_nan() {
                        Ordering::Greater
                    } else {
                        if n > nn {
                            Ordering::Greater
                        } else if n < nn {
                            Ordering::Less
                        } else {
                            Ordering::Equal
                        }
                    }
                }
                Value::Text(tt) => n.to_string().cmp(tt),
                Value::Boolean(bb) => n.is_nan().cmp(bb),
            },
            Value::Text(t) => match other {
                Value::Number(nn) => t.cmp(&nn.to_string()),
                Value::Text(tt) => t.cmp(tt),
                Value::Boolean(bb) => t.cmp(&bb.to_string()),
            },
            Value::Boolean(b) => match other {
                Value::Number(nn) => b.cmp(&!nn.is_nan()),
                Value::Text(tt) => {
                    let ttt = tt.to_lowercase();
                    b.cmp(&(ttt == "true" || ttt == "yes"))
                }
                Value::Boolean(bb) => b.cmp(bb),
            },
        }
    }
}
