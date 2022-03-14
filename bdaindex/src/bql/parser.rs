use super::scanner::*;
use super::{Rational, Value, BQL};
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
enum Step {
    Initial,
    Field,
    In,
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
    Final,
}

#[derive(Debug)]
enum Op {
    OpenPar,
    ClosePar,
    Or,
    And,
    Not,
    Ast(BQL),
}

pub fn parse(s: &str) -> Result<BQL, String> {
    let mut field: String = String::from("");
    let mut step = Step::Initial;
    let mut nodes: Vec<Op> = Vec::new();
    let tokens = scan(s);
    let mut it = tokens.iter().peekable();
    while let Some(tok) = scan_ignore_spaces(&mut it) {
        match step {
            Step::Initial => match tok {
                Token::LtParentheses => {
                    it.next();
                    nodes.push(Op::OpenPar)
                }
                Token::Not => {
                    it.next();
                    nodes.push(Op::Not);
                }
                Token::Ident(lit) | Token::Text(lit) => {
                    it.next();
                    field = lit;
                    step = Step::Field
                }
                Token::All => {
                    it.next();
                    nodes.push(Op::Ast(BQL::IsPresent));
                    step = Step::Final
                }
                _ => {
                    return Err(format!(
                        "expected IDENT | TEXT | ALL | LTParentheses | Not but got {:?}",
                        tok
                    ))
                }
            },
            Step::Field => match tok {
                Token::Not => {
                    it.next();
                    nodes.push(Op::Not);
                }
                Token::In => {
                    it.next();
                    step = Step::In;
                }
                Token::Eq => {
                    it.next();
                    step = Step::Eq;
                }
                Token::Ne => {
                    it.next();
                    nodes.push(Op::Not);
                    step = Step::Eq;
                }
                Token::Lt => {
                    it.next();
                    step = Step::Lt;
                }
                Token::Le => {
                    it.next();
                    step = Step::Le
                }
                Token::Gt => {
                    it.next();
                    step = Step::Gt
                }
                Token::Ge => {
                    it.next();
                    step = Step::Ge
                }
                Token::Or | Token::And | Token::Eof | Token::RtParentheses => {
                    nodes.push(Op::Ast(BQL::IsDefined {
                        field: field.clone(),
                    }));
                    step = Step::Final;
                }
                _ => {
                    return Err(format!(
                        "expected NOT|IN|EQ|NE|LT|LTE|GT|GTE|OR|AND|EOF|RtParentheses but got {:?}",
                        tok
                    ))
                }
            },
            Step::In => match tok {
                Token::All => {
                    it.next();
                    nodes.push(Op::Ast(BQL::All {
                        field: field.clone(),
                        values: scan_values(&mut it)?,
                    }));
                    step = Step::Final
                }
                Token::Any => {
                    it.next();
                    nodes.push(Op::Ast(BQL::Any {
                        field: field.clone(),
                        values: scan_values(&mut it)?,
                    }));
                    step = Step::Final
                }
                _ => return Err(format!("expected ALL|ANY but got {:?}", tok)),
            },
            Step::Eq => match tok {
                Token::Text(t) => {
                    it.next();
                    nodes.push(Op::Ast(BQL::Eq {
                        field: field.clone(),
                        value: Value::Text(t),
                    }));
                    step = Step::Final
                }
                Token::Number(n) => {
                    it.next();
                    nodes.push(Op::Ast(BQL::Eq {
                        field: field.clone(),
                        value: Value::Rational(Rational::from(n)),
                    }));
                    step = Step::Final
                }
                Token::True => {
                    it.next();
                    nodes.push(Op::Ast(BQL::Eq {
                        field: field.clone(),
                        value: Value::Boolean(true),
                    }));
                    step = Step::Final
                }
                Token::False => {
                    it.next();
                    nodes.push(Op::Ast(BQL::Eq {
                        field: field.clone(),
                        value: Value::Boolean(false),
                    }));
                    step = Step::Final
                }
                Token::None => {
                    it.next();
                    nodes.push(Op::Ast(BQL::Eq {
                        field: field.clone(),
                        value: Value::Bottom,
                    }));
                    step = Step::Final
                }
                Token::Defined => {
                    it.next();
                    nodes.push(Op::Ast(BQL::IsDefined {
                        field: field.clone(),
                    }));
                    step = Step::Final
                }
                Token::Not => {
                    it.next();
                    nodes.push(Op::Not);
                    step = Step::Eq
                }
                _ => {
                    return Err(format!(
                        "expected TEXT|NUMBER|NONE|TRUE|FALSE|DEFINED|NOT but got {:?}",
                        tok
                    ))
                }
            },
            Step::Lt => match tok {
                Token::Text(t) => {
                    it.next();
                    nodes.push(Op::Ast(BQL::LT {
                        field: field.clone(),
                        value: Value::Text(t),
                    }));
                    step = Step::Final
                }
                Token::Number(n) => {
                    it.next();
                    nodes.push(Op::Ast(BQL::LT {
                        field: field.clone(),
                        value: Value::Rational(Rational::from(n)),
                    }));
                    step = Step::Final
                }
                _ => return Err(format!("expected TEXT|NUMBER but got {:?}", tok)),
            },
            Step::Le => match tok {
                Token::Text(t) => {
                    it.next();
                    nodes.push(Op::Ast(BQL::LE {
                        field: field.clone(),
                        value: Value::Text(t),
                    }));
                    step = Step::Final
                }
                Token::Number(n) => {
                    it.next();
                    nodes.push(Op::Ast(BQL::LE {
                        field: field.clone(),
                        value: Value::Rational(Rational::from(n)),
                    }));
                    step = Step::Final
                }
                _ => return Err(format!("expected TEXT|NUMBER but got {:?}", tok)),
            },
            Step::Gt => match tok {
                Token::Text(t) => {
                    it.next();
                    nodes.push(Op::Ast(BQL::GT {
                        field: field.clone(),
                        value: Value::Text(t),
                    }));
                    step = Step::Final
                }
                Token::Number(n) => {
                    it.next();
                    nodes.push(Op::Ast(BQL::GT {
                        field: field.clone(),
                        value: Value::Rational(Rational::from(n)),
                    }));
                    step = Step::Final
                }
                _ => return Err(format!("expected TEXT|NUMBER but got {:?}", tok)),
            },
            Step::Ge => match tok {
                Token::Text(t) => {
                    it.next();
                    nodes.push(Op::Ast(BQL::GE {
                        field: field.clone(),
                        value: Value::Text(t),
                    }));
                    step = Step::Final
                }
                Token::Number(n) => {
                    it.next();
                    nodes.push(Op::Ast(BQL::GE {
                        field: field.clone(),
                        value: Value::Rational(Rational::from(n)),
                    }));
                    step = Step::Final
                }
                _ => return Err(format!("expected TEXT|NUMBER but got {:?}", tok)),
            },
            Step::Final => match tok {
                Token::RtParentheses => {
                    it.next();
                    nodes.push(Op::ClosePar)
                }
                Token::LtParentheses => {
                    it.next();
                    nodes.push(Op::OpenPar);
                    step = Step::Initial;
                }
                Token::Or => {
                    it.next();
                    nodes.push(Op::Or);
                    step = Step::Initial;
                }
                Token::And => {
                    it.next();
                    nodes.push(Op::And);
                    step = Step::Initial;
                }
                Token::Eof => {
                    return solve_nodes(&nodes);
                }
                _ => {
                    return Err(format!(
                        "expected OR|AND|EOF|LTParentheses|RTParenthese but got {:?}",
                        tok
                    ))
                }
            },
        }
    }
    Err("did not finished well".to_owned())
}

//https://en.wikipedia.org/wiki/Shunting-yard_algorithm
fn solve_nodes(nodes: &Vec<Op>) -> Result<BQL, String> {
    let mut ast_stack: Vec<BQL> = Vec::new();
    let mut op_stack: Vec<Op> = Vec::new();
    let mut it = nodes.iter();
    while let Some(op) = it.next() {
        match op {
            Op::Ast(ast) => {
                ast_stack.push(ast.clone());
            }
            Op::OpenPar => {
                op_stack.push(Op::OpenPar);
            }
            Op::ClosePar => {
                while let Some(o) = op_stack.pop() {
                    match o {
                        Op::OpenPar => break,
                        Op::ClosePar => {
                            return Err("not expected a close parentehese".to_owned());
                        }
                        Op::Or => {
                            let a = ast_stack.pop().ok_or("expected a value")?;
                            let b = ast_stack.pop().ok_or("expected a value")?;
                            ast_stack.push(BQL::Or(Box::new(a), Box::new(b)));
                        }
                        Op::And => {
                            let a = ast_stack.pop().ok_or("expected a value")?;
                            let b = ast_stack.pop().ok_or("expected a value")?;
                            ast_stack.push(BQL::And(Box::new(a), Box::new(b)));
                        }
                        Op::Not => {
                            let a = ast_stack.pop().ok_or("expected a value")?;
                            ast_stack.push(BQL::Not(Box::new(a)))
                        }
                        Op::Ast(_) => {
                            return Err("not expected an ast".to_owned());
                        }
                    }
                }
            }
            Op::Or => {
                while let Some(o) = op_stack.pop() {
                    match o {
                        Op::Not => {
                            let a = ast_stack.pop().ok_or("expected a value")?;
                            ast_stack.push(BQL::Not(Box::new(a)));
                        }
                        Op::And => {
                            let a = ast_stack.pop().ok_or("expected a value")?;
                            let b = ast_stack.pop().ok_or("expected a value")?;
                            ast_stack.push(BQL::And(Box::new(a), Box::new(b)));
                        }
                        _ => {
                            op_stack.push(o);
                            break;
                        }
                    }
                }
                op_stack.push(Op::Or)
            }
            Op::And => {
                while let Some(o) = op_stack.pop() {
                    match o {
                        Op::Not => {
                            let a = ast_stack.pop().ok_or("expected a value")?;
                            ast_stack.push(BQL::Not(Box::new(a)));
                        }
                        Op::And => {
                            let a = ast_stack.pop().ok_or("expected a value")?;
                            let b = ast_stack.pop().ok_or("expected a value")?;
                            ast_stack.push(BQL::And(Box::new(a), Box::new(b)));
                        }
                        _ => {
                            op_stack.push(o);
                            break;
                        }
                    }
                }
                op_stack.push(Op::And)
            }
            Op::Not => op_stack.push(Op::Not),
        }
    }
    while let Some(op) = op_stack.pop() {
        match op {
            Op::Or => {
                let a = ast_stack.pop().ok_or("expected a value")?;
                let b = ast_stack.pop().ok_or("expected a value")?;
                ast_stack.push(BQL::Or(Box::new(a), Box::new(b)));
            }
            Op::And => {
                let a = ast_stack.pop().ok_or("expected a value")?;
                let b = ast_stack.pop().ok_or("expected a value")?;
                ast_stack.push(BQL::And(Box::new(a), Box::new(b)));
            }
            Op::Not => {
                let a = ast_stack.pop().ok_or("expected a value")?;
                ast_stack.push(BQL::Not(Box::new(a)))
            }
            _ => {
                return Err(format!("expected OR | AND but got {:?}", op));
            }
        }
    }
    ast_stack.pop().ok_or("could not compute ast".to_owned())
}

fn scan_values<'a, T: Iterator<Item = &'a Token>>(
    it: &mut Peekable<T>,
) -> Result<Vec<Value>, String> {
    match scan_ignore_spaces(it) {
        Some(Token::LtBracket) => it.next(),
        _ => return Err("value expression did not start with left brackets".to_owned()),
    };
    let mut values: Vec<Value> = Vec::new();
    while let Some(tok) = scan_ignore_spaces(it) {
        match tok {
            Token::RtBracket => {
                it.next();
                return Ok(values);
            }
            Token::Text(t) => {
                it.next();
                values.push(Value::Text(t));
            }
            Token::Number(n) => {
                it.next();
                values.push(Value::Rational(Rational::from(n)));
            }
            Token::True => {
                it.next();
                values.push(Value::Boolean(true));
            }
            Token::False => {
                it.next();
                values.push(Value::Boolean(false));
            }
            Token::None => {
                it.next();
                values.push(Value::Bottom);
            }
            Token::Comma => {
                it.next();
            }
            _ => {
                return Err(format!(
                    "expected RtBracket|Text|Number|True|False|None|Comma but got {:?}",
                    tok
                ))
            }
        }
    }
    Err("scan_values did not finish well".to_owned())
}

fn scan_ignore_spaces<'a, T: Iterator<Item = &'a Token>>(it: &mut Peekable<T>) -> Option<Token> {
    while let Some(t) = it.peek() {
        match t {
            Token::Ws => {
                it.next();
                continue;
            }
            _ => return Some((*t).clone()),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scan_ignore_spaces() {
        let v = scan("    .field     .another.field");
        let mut it = v.iter().peekable();

        assert_eq!(
            scan_ignore_spaces(&mut it).unwrap(),
            Token::Ident(".field".to_owned())
        );
        assert_eq!(*it.next().unwrap(), Token::Ident(".field".to_owned()));

        assert_eq!(
            scan_ignore_spaces(&mut it).unwrap(),
            Token::Ident(".another.field".to_owned())
        );
        assert_eq!(
            *it.next().unwrap(),
            Token::Ident(".another.field".to_owned())
        );
    }

    #[test]
    fn test_all_relation() {
        assert_eq!(BQL::IsPresent, parse(r#"all"#).unwrap());
    }
    #[test]
    fn test_defined_relation_single_field() {
        assert_eq!(
            BQL::IsDefined {
                field: "field".to_owned(),
            },
            parse(r#"field"#).unwrap()
        );
    }
    #[test]
    fn test_not_defined_relation_with_exclamation() {
        assert_eq!(
            BQL::Not(Box::new(BQL::IsDefined {
                field: "field".to_owned(),
            })),
            parse(r#"!field"#).unwrap()
        );
    }
    #[test]
    fn test_not_defined_relation_with_exclamation_and_space() {
        assert_eq!(
            BQL::Not(Box::new(BQL::IsDefined {
                field: "field".to_owned(),
            })),
            parse(r#"! field"#).unwrap()
        );
    }

    #[test]
    fn test_not_defined_relation_with_not() {
        assert_eq!(
            BQL::Not(Box::new(BQL::IsDefined {
                field: "field".to_owned(),
            })),
            parse(r#"not field"#).unwrap()
        );
    }

    #[test]
    fn test_defined_relation() {
        assert_eq!(
            BQL::IsDefined {
                field: "field".to_owned(),
            },
            parse(r#"field == defined"#).unwrap()
        );
    }
    #[test]
    fn test_not_defined_relation() {
        assert_eq!(
            BQL::Not(Box::new(BQL::IsDefined {
                field: "field".to_owned(),
            })),
            parse(r#"field != defined"#).unwrap()
        );
    }

    #[test]
    fn test_not_defined_relation_single_quoted() {
        assert_eq!(
            BQL::Not(Box::new(BQL::IsDefined {
                field: "field".to_owned(),
            })),
            parse(r#"'field' != defined"#).unwrap()
        );
    }
    #[test]
    fn test_defined_relation_double_quoted() {
        assert_eq!(
            BQL::IsDefined {
                field: "field".to_owned(),
            },
            parse(r#""field" == defined"#).unwrap()
        );
    }

    #[test]
    fn test_defined_relation_and_eq() {
        assert_eq!(
            BQL::IsDefined {
                field: "field".to_owned(),
            },
            parse(r#"field eq defined"#).unwrap()
        );
    }

    #[test]
    fn test_not_defined_relation_and_ne() {
        assert_eq!(
            BQL::Not(Box::new(BQL::IsDefined {
                field: "field".to_owned(),
            })),
            parse(r#"field ne defined"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_eq_str() {
        assert_eq!(
            BQL::Eq {
                field: "field".to_owned(),
                value: Value::Text("abc".to_owned())
            },
            parse(r#"field eq "abc""#).unwrap()
        );
    }
    #[test]
    fn test_eq_relation_with_ne_str() {
        assert_eq!(
            BQL::Not(Box::new(BQL::Eq {
                field: "field".to_owned(),
                value: Value::Text("abc".to_owned())
            })),
            parse(r#"field ne "abc""#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_eq_number() {
        assert_eq!(
            BQL::Eq {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field eq 5"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_eq_negative_number() {
        assert_eq!(
            BQL::Eq {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(-5.0 as f64))
            },
            parse(r#"field eq -5"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_ne_number() {
        assert_eq!(
            BQL::Not(Box::new(BQL::Eq {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            })),
            parse(r#"field ne 5"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_eq_true() {
        assert_eq!(
            BQL::Eq {
                field: "field".to_owned(),
                value: Value::Boolean(true)
            },
            parse(r#"field eq true"#).unwrap()
        );
    }
    #[test]
    fn test_eq_relation_with_ne_true() {
        assert_eq!(
            BQL::Not(Box::new(BQL::Eq {
                field: "field".to_owned(),
                value: Value::Boolean(true)
            })),
            parse(r#"field ne true"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_eq_false() {
        assert_eq!(
            BQL::Eq {
                field: "field".to_owned(),
                value: Value::Boolean(false)
            },
            parse(r#"field eq false"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_is_false() {
        assert_eq!(
            BQL::Eq {
                field: "field".to_owned(),
                value: Value::Boolean(false)
            },
            parse(r#"field is false"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_ne_false() {
        assert_eq!(
            BQL::Not(Box::new(BQL::Eq {
                field: "field".to_owned(),
                value: Value::Boolean(false)
            })),
            parse(r#"field ne false"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_is_null() {
        assert_eq!(
            BQL::Eq {
                field: "field".to_owned(),
                value: Value::Bottom,
            },
            parse(r#"field is null"#).unwrap()
        );
    }
    #[test]
    fn test_eq_relation_with_is_nil() {
        assert_eq!(
            BQL::Eq {
                field: "field".to_owned(),
                value: Value::Bottom,
            },
            parse(r#"field is nil"#).unwrap()
        );
    }
    #[test]
    fn test_eq_relation_with_is_nothing() {
        assert_eq!(
            BQL::Eq {
                field: "field".to_owned(),
                value: Value::Bottom,
            },
            parse(r#"field is nothing"#).unwrap()
        );
    }

    #[test]
    fn test_lt_relation_with_symbol() {
        assert_eq!(
            BQL::LT {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field < 5"#).unwrap()
        );
    }
    #[test]
    fn test_lt_relation_with_str() {
        assert_eq!(
            BQL::LT {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field lt 5"#).unwrap()
        );
    }

    #[test]
    fn test_lte_relation_with_symbol() {
        assert_eq!(
            BQL::LE {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field <= 5"#).unwrap()
        );
    }
    #[test]
    fn test_lte_relation_with_str() {
        assert_eq!(
            BQL::LE {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field lte 5"#).unwrap()
        );
    }

    #[test]
    fn test_gt_relation_with_symbol() {
        assert_eq!(
            BQL::GT {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field > 5"#).unwrap()
        );
    }
    #[test]
    fn test_gt_relation_with_str() {
        assert_eq!(
            BQL::GT {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field gt 5"#).unwrap()
        );
    }

    #[test]
    fn test_gte_relation_with_symbol() {
        assert_eq!(
            BQL::GE {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field >= 5"#).unwrap()
        );
    }
    #[test]
    fn test_gte_relation_with_str() {
        assert_eq!(
            BQL::GE {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field gte 5"#).unwrap()
        );
    }

    #[test]
    fn test_in_all_relation_mixed_values() {
        assert_eq!(
            BQL::All {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
            },
            parse(r#"field in all ["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_in_all_relation_mixed_values_and_no_spaces() {
        assert_eq!(
            BQL::All {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
            },
            parse(r#"field#all["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_not_in_all_relation_mixed_values() {
        assert_eq!(
            BQL::Not(Box::new(BQL::All {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
            })),
            parse(r#"field not in all ["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_not_in_all_relation_mixed_values_and_no_spaces() {
        assert_eq!(
            BQL::Not(Box::new(BQL::All {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
            })),
            parse(r#"field!@all["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_in_any_relation_mixed_values() {
        assert_eq!(
            BQL::Any {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
            },
            parse(r#"field in any ["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_in_any_relation_mixed_values_and_no_spaces() {
        assert_eq!(
            BQL::Any {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
            },
            parse(r#"field#any["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_not_in_any_relation_mixed_values() {
        assert_eq!(
            BQL::Not(Box::new(BQL::Any {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
            })),
            parse(r#"field not in any ["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_not_in_any_relation_mixed_values_and_no_spaces() {
        assert_eq!(
            BQL::Not(Box::new(BQL::Any {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
            })),
            parse(r#"field!@any["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_union_of_three_predicates() {
        assert_eq!(
            BQL::Or(
                Box::new(BQL::Or(
                    Box::new(BQL::Eq {
                        field: "field".to_owned(),
                        value: Value::Rational(Rational::from(42.0 as f64)),
                    }),
                    Box::new(BQL::Eq {
                        field: "field".to_owned(),
                        value: Value::Bottom,
                    }),
                )),
                Box::new(BQL::IsDefined {
                    field: "field".to_owned(),
                }),
            ),
            parse(r#"field == defined  or  field == null or field == 42"#).unwrap()
        );
    }

    #[test]
    fn test_intersection_of_three_predicates() {
        assert_eq!(
            BQL::And(
                Box::new(BQL::Eq {
                    field: "field".to_owned(),
                    value: Value::Rational(Rational::from(42.0 as f64)),
                }),
                Box::new(BQL::And(
                    Box::new(BQL::Eq {
                        field: "field".to_owned(),
                        value: Value::Bottom,
                    }),
                    Box::new(BQL::IsDefined {
                        field: "field".to_owned(),
                    }),
                )),
            ),
            parse(r#"field == defined  and  field == null and field == 42"#).unwrap()
        );
    }
    #[test]
    fn test_intersection_and_union_with_parentheses() {
        assert_eq!(
            BQL::And(
                Box::new(BQL::Or(
                    Box::new(BQL::Eq {
                        field: "field".to_owned(),
                        value: Value::Rational(Rational::from(42.0 as f64)),
                    }),
                    Box::new(BQL::Eq {
                        field: "field".to_owned(),
                        value: Value::Bottom,
                    }),
                )),
                Box::new(BQL::IsDefined {
                    field: "field".to_owned(),
                })
            ),
            parse(r#"(field == defined ) and  ( field == null or field == 42 ) "#).unwrap()
        );
    }

    #[test]
    fn test_intersection_and_union_with_parenthesesand_ne() {
        assert_eq!(
            BQL::And(
                Box::new(BQL::Not(Box::new(BQL::Or(
                    Box::new(BQL::Not(Box::new(BQL::LT {
                        field: "field".to_owned(),
                        value: Value::Rational(Rational::from(42.0 as f64)),
                    }))),
                    Box::new(BQL::Eq {
                        field: "field".to_owned(),
                        value: Value::Bottom,
                    }),
                )))),
                Box::new(BQL::Not(Box::new(BQL::IsDefined {
                    field: "field".to_owned(),
                })))
            ),
            parse(r#"!(field)&&!(field==null||!field<42)"#).unwrap()
        );
    }
}
