use std::iter::Peekable;

use super::ast::*;
use super::scanner::*;

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
    PreFinal,
    Final,
}

pub fn parse(s: &str) -> Result<Ast, String> {
    let mut negate = false;
    let mut intersetFlag = false;
    let mut unionFlag = false;
    let mut fname: String = String::from("");
    let mut value: Value = Value::Text(String::from(""));
    let mut step = Step::Initial;
    let mut stack: Vec<Ast> = Vec::new();
    let tokens = scan(s);
    let mut it = tokens.iter().peekable();

    while let Some(tok) = scan_ignore_spaces(&mut it) {
        let ftok = tok.clone();
        match step {
            Step::Initial => match tok {
                Token::Not => {
                    negate = !negate;
                    continue;
                }
                Token::Ident(lit) | Token::Text(lit) => {
                    fname = lit;
                    step = Step::Field
                }
                Token::All => {
                    stack.push(Ast::All);
                    step = Step::Final
                }
                _ => {
                    return Err(format!(
                        "expected IDENT | TEXT | NOT | ALL but got {:?}",
                        tok
                    ))
                }
            },
            Step::Field => match tok {
                Token::Not => {
                    negate = !negate;
                    continue;
                }
                Token::In => step = Step::InRelation,
                Token::Eq => step = Step::EqRelation,
                Token::Ne => {
                    negate = !negate;
                    step = Step::EqRelation;
                }
                Token::Lt => step = Step::LtRelation,
                Token::Lte => step = Step::LteRelation,
                Token::Gt => step = Step::GtRelation,
                Token::Gte => step = Step::GteRelation,
                Token::Or | Token::And | Token::Eof => {
                    stack.push(Ast::Defined {
                        fname: fname.clone(),
                        negate: negate,
                    });
                    step = Step::Final;
                }
                _ => {
                    return Err(format!(
                        "expected NOT|IN|EQ|NE|LT|LTE|GT|GTE|OR|AND|EOF but got {:?}",
                        tok
                    ))
                }
            },
            Step::InRelation => todo!(),
            Step::EqRelation => todo!(),
            Step::LtRelation => todo!(),
            Step::LteRelation => todo!(),
            Step::GtRelation => todo!(),
            Step::GteRelation => todo!(),
            Step::PreFinal => step = Step::Final,
            Step::Final => step = Step::Final,
        }

        if Step::Final == step {
            // TODO: merge on intersect flag true function
            match ftok {
                Token::Eof => {
                    //TODO: merge on or flag true function
                    return match stack.pop() {
                        Some(ast) => Ok(ast),
                        None => Err("did not finished well".to_owned()),
                    };
                }
                Token::Or => {
                    //TODO: merge on or flag true function
                    unionFlag = true;
                    negate = false;
                    step = Step::Initial;
                }
                Token::And => {
                    intersetFlag = true;
                    negate = false;
                    step = Step::Initial;
                }
                _ => return Err(format!("expected OR|AND|EOF but got {:?}", ftok)),
            }
        }
    }
    Err("did not finished well".to_owned())
}

fn scan_ignore_spaces<'a, T: Iterator<Item = &'a Token>>(it: &mut Peekable<T>) -> Option<Token> {
    while let Some(t) = it.next() {
        match t {
            Token::Ws => continue,
            _ => return Some(t.clone()),
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
        assert_eq!(
            scan_ignore_spaces(&mut it).unwrap(),
            Token::Ident(".another.field".to_owned())
        );
    }
}
