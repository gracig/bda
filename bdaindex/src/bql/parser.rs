use super::scanner::*;
use super::{Ast, Rational, Value};
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
enum Step {
    Initial,
    Field,
    InRelation,
    EqRelation,
    LtRelation,
    LteRelation,
    GtRelation,
    GteRelation,
    Final,
}

#[derive(Debug)]
enum Op {
    OpenPar,
    ClosePar,
    Or,
    And,
    Ast(Ast),
}

pub fn parse(s: &str) -> Result<Ast, String> {
    let mut negate = false;
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
                    match scan_ignore_spaces(&mut it) {
                        Some(Token::Ident(_)) => {
                            negate = !negate;
                        }
                        _ => {
                            return Err(format!("expected IDENT but got {:?}", tok));
                        }
                    }
                }
                Token::Ident(lit) | Token::Text(lit) => {
                    it.next();
                    field = lit;
                    step = Step::Field
                }
                Token::All => {
                    it.next();
                    nodes.push(Op::Ast(Ast::All));
                    step = Step::Final
                }
                _ => {
                    return Err(format!(
                        "expected IDENT | TEXT | ALL | LTParentheses but got {:?}",
                        tok
                    ))
                }
            },
            Step::Field => match tok {
                Token::Not => {
                    it.next();
                    negate = !negate;
                }
                Token::In => {
                    it.next();
                    step = Step::InRelation;
                }
                Token::Eq => {
                    it.next();
                    step = Step::EqRelation;
                }
                Token::Ne => {
                    it.next();
                    negate = !negate;
                    step = Step::EqRelation;
                }
                Token::Lt => {
                    it.next();
                    step = Step::LtRelation;
                }
                Token::Lte => {
                    it.next();
                    step = Step::LteRelation
                }
                Token::Gt => {
                    it.next();
                    step = Step::GtRelation
                }
                Token::Gte => {
                    it.next();
                    step = Step::GteRelation
                }
                Token::Or | Token::And | Token::Eof | Token::RtParentheses => {
                    nodes.push(Op::Ast(Ast::Defined {
                        field: field.clone(),
                        negate: negate,
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
            Step::InRelation => match tok {
                Token::All => {
                    it.next();
                    nodes.push(Op::Ast(Ast::ContainsAll {
                        field: field.clone(),
                        values: scan_values(&mut it)?,
                        negate: negate,
                    }));
                    step = Step::Final
                }
                Token::Any => {
                    it.next();
                    nodes.push(Op::Ast(Ast::ContainsAny {
                        field: field.clone(),
                        values: scan_values(&mut it)?,
                        negate: negate,
                    }));
                    step = Step::Final
                }
                _ => return Err(format!("expected ALL|ANY but got {:?}", tok)),
            },
            Step::EqRelation => match tok {
                Token::Text(t) => {
                    it.next();
                    nodes.push(Op::Ast(Ast::Equal {
                        field: field.clone(),
                        value: Value::Text(t),
                        negate: negate,
                    }));
                    step = Step::Final
                }
                Token::Number(n) => {
                    it.next();
                    nodes.push(Op::Ast(Ast::Equal {
                        field: field.clone(),
                        value: Value::Rational(Rational::from(n)),
                        negate: negate,
                    }));
                    step = Step::Final
                }
                Token::True => {
                    it.next();
                    nodes.push(Op::Ast(Ast::Equal {
                        field: field.clone(),
                        value: Value::Boolean(true),
                        negate: negate,
                    }));
                    step = Step::Final
                }
                Token::False => {
                    it.next();
                    nodes.push(Op::Ast(Ast::Equal {
                        field: field.clone(),
                        value: Value::Boolean(false),
                        negate: negate,
                    }));
                    step = Step::Final
                }
                Token::None => {
                    it.next();
                    nodes.push(Op::Ast(Ast::Equal {
                        field: field.clone(),
                        value: Value::Bottom,
                        negate: negate,
                    }));
                    step = Step::Final
                }
                Token::Defined => {
                    it.next();
                    nodes.push(Op::Ast(Ast::Defined {
                        field: field.clone(),
                        negate: negate,
                    }));
                    step = Step::Final
                }
                Token::Not => {
                    it.next();
                    negate = !negate;
                    step = Step::EqRelation
                }
                _ => {
                    return Err(format!(
                        "expected TEXT|NUMBER|NONE|TRUE|FALSE|DEFINED|NOT but got {:?}",
                        tok
                    ))
                }
            },
            Step::LtRelation => match tok {
                Token::Text(t) => {
                    it.next();
                    nodes.push(Op::Ast(Ast::LessThan {
                        field: field.clone(),
                        value: Value::Text(t),
                    }));
                    step = Step::Final
                }
                Token::Number(n) => {
                    it.next();
                    nodes.push(Op::Ast(Ast::LessThan {
                        field: field.clone(),
                        value: Value::Rational(Rational::from(n)),
                    }));
                    step = Step::Final
                }
                _ => return Err(format!("expected TEXT|NUMBER but got {:?}", tok)),
            },
            Step::LteRelation => match tok {
                Token::Text(t) => {
                    it.next();
                    nodes.push(Op::Ast(Ast::LessThanOrEqual {
                        field: field.clone(),
                        value: Value::Text(t),
                    }));
                    step = Step::Final
                }
                Token::Number(n) => {
                    it.next();
                    nodes.push(Op::Ast(Ast::LessThanOrEqual {
                        field: field.clone(),
                        value: Value::Rational(Rational::from(n)),
                    }));
                    step = Step::Final
                }
                _ => return Err(format!("expected TEXT|NUMBER but got {:?}", tok)),
            },
            Step::GtRelation => match tok {
                Token::Text(t) => {
                    it.next();
                    nodes.push(Op::Ast(Ast::GreaterThan {
                        field: field.clone(),
                        value: Value::Text(t),
                    }));
                    step = Step::Final
                }
                Token::Number(n) => {
                    it.next();
                    nodes.push(Op::Ast(Ast::GreaterThan {
                        field: field.clone(),
                        value: Value::Rational(Rational::from(n)),
                    }));
                    step = Step::Final
                }
                _ => return Err(format!("expected TEXT|NUMBER but got {:?}", tok)),
            },
            Step::GteRelation => match tok {
                Token::Text(t) => {
                    it.next();
                    nodes.push(Op::Ast(Ast::GreaterThanOrEqual {
                        field: field.clone(),
                        value: Value::Text(t),
                    }));
                    step = Step::Final
                }
                Token::Number(n) => {
                    it.next();
                    nodes.push(Op::Ast(Ast::GreaterThanOrEqual {
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
                    negate = false;
                    step = Step::Initial;
                }
                Token::And => {
                    it.next();
                    nodes.push(Op::And);
                    negate = false;
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
fn solve_nodes(nodes: &Vec<Op>) -> Result<Ast, String> {
    let mut ast_stack: Vec<Ast> = Vec::new();
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
                            ast_stack.push(Ast::Union(Box::new(a), Box::new(b)));
                        }
                        Op::And => {
                            let a = ast_stack.pop().ok_or("expected a value")?;
                            let b = ast_stack.pop().ok_or("expected a value")?;
                            ast_stack.push(Ast::Intersection(Box::new(a), Box::new(b)));
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
                        Op::And => {
                            let a = ast_stack.pop().ok_or("expected a value")?;
                            let b = ast_stack.pop().ok_or("expected a value")?;
                            ast_stack.push(Ast::Intersection(Box::new(a), Box::new(b)));
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
                        Op::And => {
                            let a = ast_stack.pop().ok_or("expected a value")?;
                            let b = ast_stack.pop().ok_or("expected a value")?;
                            ast_stack.push(Ast::Intersection(Box::new(a), Box::new(b)));
                        }
                        _ => {
                            op_stack.push(o);
                            break;
                        }
                    }
                }
                op_stack.push(Op::And)
            }
        }
    }
    while let Some(op) = op_stack.pop() {
        match op {
            Op::Or => {
                let a = ast_stack.pop().ok_or("expected a value")?;
                let b = ast_stack.pop().ok_or("expected a value")?;
                ast_stack.push(Ast::Union(Box::new(a), Box::new(b)));
            }
            Op::And => {
                let a = ast_stack.pop().ok_or("expected a value")?;
                let b = ast_stack.pop().ok_or("expected a value")?;
                ast_stack.push(Ast::Intersection(Box::new(a), Box::new(b)));
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
        assert_eq!(Ast::All, parse(r#"all"#).unwrap());
    }
    #[test]
    fn test_defined_relation_single_field() {
        assert_eq!(
            Ast::Defined {
                field: "field".to_owned(),
                negate: false
            },
            parse(r#"field"#).unwrap()
        );
    }
    #[test]
    fn test_not_defined_relation_with_exclamation() {
        assert_eq!(
            Ast::Defined {
                field: "field".to_owned(),
                negate: true
            },
            parse(r#"!field"#).unwrap()
        );
    }
    #[test]
    fn test_not_defined_relation_with_exclamation_and_space() {
        assert_eq!(
            Ast::Defined {
                field: "field".to_owned(),
                negate: true
            },
            parse(r#"! field"#).unwrap()
        );
    }

    #[test]
    fn test_not_defined_relation_with_not() {
        assert_eq!(
            Ast::Defined {
                field: "field".to_owned(),
                negate: true
            },
            parse(r#"not field"#).unwrap()
        );
    }

    #[test]
    fn test_defined_relation() {
        assert_eq!(
            Ast::Defined {
                field: "field".to_owned(),
                negate: false
            },
            parse(r#"field == defined"#).unwrap()
        );
    }
    #[test]
    fn test_not_defined_relation() {
        assert_eq!(
            Ast::Defined {
                field: "field".to_owned(),
                negate: true
            },
            parse(r#"field != defined"#).unwrap()
        );
    }

    #[test]
    fn test_not_defined_relation_single_quoted() {
        assert_eq!(
            Ast::Defined {
                field: "field".to_owned(),
                negate: true
            },
            parse(r#"'field' != defined"#).unwrap()
        );
    }
    #[test]
    fn test_defined_relation_double_quoted() {
        assert_eq!(
            Ast::Defined {
                field: "field".to_owned(),
                negate: false
            },
            parse(r#""field" == defined"#).unwrap()
        );
    }

    #[test]
    fn test_defined_relation_and_eq() {
        assert_eq!(
            Ast::Defined {
                field: "field".to_owned(),
                negate: false
            },
            parse(r#"field eq defined"#).unwrap()
        );
    }

    #[test]
    fn test_not_defined_relation_and_ne() {
        assert_eq!(
            Ast::Defined {
                field: "field".to_owned(),
                negate: true
            },
            parse(r#"field ne defined"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_eq_str() {
        assert_eq!(
            Ast::Equal {
                field: "field".to_owned(),
                negate: false,
                value: Value::Text("abc".to_owned())
            },
            parse(r#"field eq "abc""#).unwrap()
        );
    }
    #[test]
    fn test_eq_relation_with_ne_str() {
        assert_eq!(
            Ast::Equal {
                field: "field".to_owned(),
                negate: true,
                value: Value::Text("abc".to_owned())
            },
            parse(r#"field ne "abc""#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_eq_number() {
        assert_eq!(
            Ast::Equal {
                field: "field".to_owned(),
                negate: false,
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field eq 5"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_eq_negative_number() {
        assert_eq!(
            Ast::Equal {
                field: "field".to_owned(),
                negate: false,
                value: Value::Rational(Rational::from(-5.0 as f64))
            },
            parse(r#"field eq -5"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_ne_number() {
        assert_eq!(
            Ast::Equal {
                field: "field".to_owned(),
                negate: true,
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field ne 5"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_eq_true() {
        assert_eq!(
            Ast::Equal {
                field: "field".to_owned(),
                negate: false,
                value: Value::Boolean(true)
            },
            parse(r#"field eq true"#).unwrap()
        );
    }
    #[test]
    fn test_eq_relation_with_ne_true() {
        assert_eq!(
            Ast::Equal {
                field: "field".to_owned(),
                negate: true,
                value: Value::Boolean(true)
            },
            parse(r#"field ne true"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_eq_false() {
        assert_eq!(
            Ast::Equal {
                field: "field".to_owned(),
                negate: false,
                value: Value::Boolean(false)
            },
            parse(r#"field eq false"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_is_false() {
        assert_eq!(
            Ast::Equal {
                field: "field".to_owned(),
                negate: false,
                value: Value::Boolean(false)
            },
            parse(r#"field is false"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_ne_false() {
        assert_eq!(
            Ast::Equal {
                field: "field".to_owned(),
                negate: true,
                value: Value::Boolean(false)
            },
            parse(r#"field ne false"#).unwrap()
        );
    }

    #[test]
    fn test_eq_relation_with_is_null() {
        assert_eq!(
            Ast::Equal {
                field: "field".to_owned(),
                negate: false,
                value: Value::Bottom,
            },
            parse(r#"field is null"#).unwrap()
        );
    }
    #[test]
    fn test_eq_relation_with_is_nil() {
        assert_eq!(
            Ast::Equal {
                field: "field".to_owned(),
                negate: false,
                value: Value::Bottom,
            },
            parse(r#"field is nil"#).unwrap()
        );
    }
    #[test]
    fn test_eq_relation_with_is_nothing() {
        assert_eq!(
            Ast::Equal {
                field: "field".to_owned(),
                negate: false,
                value: Value::Bottom,
            },
            parse(r#"field is nothing"#).unwrap()
        );
    }

    #[test]
    fn test_lt_relation_with_symbol() {
        assert_eq!(
            Ast::LessThan {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field < 5"#).unwrap()
        );
    }
    #[test]
    fn test_lt_relation_with_str() {
        assert_eq!(
            Ast::LessThan {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field lt 5"#).unwrap()
        );
    }

    #[test]
    fn test_lte_relation_with_symbol() {
        assert_eq!(
            Ast::LessThanOrEqual {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field <= 5"#).unwrap()
        );
    }
    #[test]
    fn test_lte_relation_with_str() {
        assert_eq!(
            Ast::LessThanOrEqual {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field lte 5"#).unwrap()
        );
    }

    #[test]
    fn test_gt_relation_with_symbol() {
        assert_eq!(
            Ast::GreaterThan {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field > 5"#).unwrap()
        );
    }
    #[test]
    fn test_gt_relation_with_str() {
        assert_eq!(
            Ast::GreaterThan {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field gt 5"#).unwrap()
        );
    }

    #[test]
    fn test_gte_relation_with_symbol() {
        assert_eq!(
            Ast::GreaterThanOrEqual {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field >= 5"#).unwrap()
        );
    }
    #[test]
    fn test_gte_relation_with_str() {
        assert_eq!(
            Ast::GreaterThanOrEqual {
                field: "field".to_owned(),
                value: Value::Rational(Rational::from(5.0 as f64))
            },
            parse(r#"field gte 5"#).unwrap()
        );
    }

    #[test]
    fn test_in_all_relation_mixed_values() {
        assert_eq!(
            Ast::ContainsAll {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
                negate: false
            },
            parse(r#"field in all ["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_in_all_relation_mixed_values_and_no_spaces() {
        assert_eq!(
            Ast::ContainsAll {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
                negate: false
            },
            parse(r#"field#all["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_not_in_all_relation_mixed_values() {
        assert_eq!(
            Ast::ContainsAll {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
                negate: true
            },
            parse(r#"field not in all ["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_not_in_all_relation_mixed_values_and_no_spaces() {
        assert_eq!(
            Ast::ContainsAll {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
                negate: true
            },
            parse(r#"field!@all["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_in_any_relation_mixed_values() {
        assert_eq!(
            Ast::ContainsAny {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
                negate: false
            },
            parse(r#"field in any ["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_in_any_relation_mixed_values_and_no_spaces() {
        assert_eq!(
            Ast::ContainsAny {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
                negate: false
            },
            parse(r#"field#any["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_not_in_any_relation_mixed_values() {
        assert_eq!(
            Ast::ContainsAny {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
                negate: true
            },
            parse(r#"field not in any ["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_not_in_any_relation_mixed_values_and_no_spaces() {
        assert_eq!(
            Ast::ContainsAny {
                field: "field".to_owned(),
                values: vec![
                    Value::Text("a".to_owned()),
                    Value::Rational(Rational::from(42.05 as f64)),
                    Value::Boolean(true),
                    Value::Bottom,
                ],
                negate: true
            },
            parse(r#"field!@any["a",42.05,true,nil]"#).unwrap()
        );
    }

    #[test]
    fn test_union_of_three_predicates() {
        assert_eq!(
            Ast::Union(
                Box::new(Ast::Union(
                    Box::new(Ast::Equal {
                        field: "field".to_owned(),
                        value: Value::Rational(Rational::from(42.0 as f64)),
                        negate: false
                    }),
                    Box::new(Ast::Equal {
                        field: "field".to_owned(),
                        value: Value::Bottom,
                        negate: false
                    }),
                )),
                Box::new(Ast::Defined {
                    field: "field".to_owned(),
                    negate: false,
                }),
            ),
            parse(r#"field == defined  or  field == null or field == 42"#).unwrap()
        );
    }

    #[test]
    fn test_intersection_of_three_predicates() {
        assert_eq!(
            Ast::Intersection(
                Box::new(Ast::Equal {
                    field: "field".to_owned(),
                    value: Value::Rational(Rational::from(42.0 as f64)),
                    negate: false
                }),
                Box::new(Ast::Intersection(
                    Box::new(Ast::Equal {
                        field: "field".to_owned(),
                        value: Value::Bottom,
                        negate: false
                    }),
                    Box::new(Ast::Defined {
                        field: "field".to_owned(),
                        negate: false,
                    }),
                )),
            ),
            parse(r#"field == defined  and  field == null and field == 42"#).unwrap()
        );
    }
    #[test]
    fn test_intersection_and_union_with_parentheses() {
        assert_eq!(
            Ast::Intersection(
                Box::new(Ast::Union(
                    Box::new(Ast::Equal {
                        field: "field".to_owned(),
                        value: Value::Rational(Rational::from(42.0 as f64)),
                        negate: false
                    }),
                    Box::new(Ast::Equal {
                        field: "field".to_owned(),
                        value: Value::Bottom,
                        negate: false
                    }),
                )),
                Box::new(Ast::Defined {
                    field: "field".to_owned(),
                    negate: false
                })
            ),
            parse(r#"(field == defined ) and  ( field == null or field == 42 ) "#).unwrap()
        );
    }
}
