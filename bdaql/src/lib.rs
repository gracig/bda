mod parser;
mod scanner;

pub fn from_str(s: &str) -> Result<Ast, String> {
    return parser::parse(s);
}

#[derive(PartialEq, Debug, Clone)]
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
