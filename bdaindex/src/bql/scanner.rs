use std::str::FromStr;
use std::{iter::Peekable, num::ParseFloatError};

pub fn scan(s: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut it = s.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            ' ' => {
                it.next();
                result.push(Token::Ws);
            }
            'a'..='z' | 'A'..='Z' | '.' | '_' => {
                result.push(scan_ident(&mut it));
            }
            '0'..='9' => {
                result.push(scan_number(&mut it));
            }
            '>' | '<' | '=' | '!' => {
                result.push(scan_relation(&mut it));
            }
            '\'' | '"' | '*' => {
                result.push(scan_text(&mut it));
            }
            '-' => {
                it.next();
                match it.peek() {
                    Some('0'..='9') => {
                        if let Token::Number(n) = scan_number(&mut it) {
                            result.push(Token::Number(n * -1.0))
                        }
                    }
                    _ => result.push(Token::Illegal(c)),
                }
            }
            '&' => {
                it.next();
                match it.peek() {
                    Some('&') => {
                        it.next();
                        result.push(Token::And);
                    }
                    _ => result.push(Token::Illegal(c)),
                }
            }
            '|' => {
                it.next();
                match it.peek() {
                    Some('|') => {
                        it.next();
                        result.push(Token::Or);
                    }
                    _ => result.push(Token::Illegal(c)),
                }
            }
            ',' => {
                it.next();
                result.push(Token::Comma);
            }
            '[' => {
                it.next();
                result.push(Token::LtBracket);
            }
            ']' => {
                it.next();
                result.push(Token::RtBracket);
            }
            '(' => {
                it.next();
                result.push(Token::LtParentheses);
            }
            ')' => {
                it.next();
                result.push(Token::RtParentheses);
            }
            '#' | '@' => {
                it.next();
                result.push(Token::In);
            }
            _ => result.push(Token::Illegal(c)),
        }
        match result.last() {
            Some(Token::Illegal(_)) => break,
            Some(Token::BadNumber(_, _)) => break,
            Some(Token::BadRelation(_)) => break,
            Some(Token::UnclosedText(_)) => break,
            _ => continue,
        }
    }
    result.push(Token::Eof);
    result
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal(char),
    Eof,
    Ws,
    Comma,
    LtBracket,
    RtBracket,
    LtParentheses,
    RtParentheses,
    Ident(String),
    Number(f64),
    BadNumber(String, ParseFloatError),
    Text(String),
    UnclosedText(String),
    BadRelation(String),
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    None,
    Defined,
    True,
    False,
    In,
    All,
    Any,
    And,
    Or,
    Not,
}

fn scan_ident<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> Token {
    let mut buf = String::new();
    while let Some(&c) = it.peek() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_' => buf.push(c),
            _ => break,
        }
        it.next();
    }
    match buf.to_uppercase().as_str() {
        "NOT" => Token::Not,
        "NONE" => Token::None,
        "NULL" => Token::None,
        "NOTHING" => Token::None,
        "NIL" => Token::None,
        "DEFINED" => Token::Defined,
        "TRUE" => Token::True,
        "YES" => Token::True,
        "NO" => Token::False,
        "FALSE" => Token::False,
        "IN" => Token::In,
        "ALL" => Token::All,
        "ANY" => Token::Any,
        "AND" => Token::And,
        "OR" => Token::Or,
        "EQ" => Token::Eq,
        "NE" => Token::Ne,
        "LT" => Token::Lt,
        "LTE" => Token::Lte,
        "GT" => Token::Gt,
        "GTE" => Token::Gte,
        "IS" => Token::Eq,
        _ => Token::Ident(buf),
    }
}
fn scan_number<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> Token {
    let mut buf = String::new();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' | '.' => buf.push(c),
            _ => break,
        }
        it.next();
    }
    match f64::from_str(&buf) {
        Ok(n) => Token::Number(n),
        Err(e) => Token::BadNumber(buf, e),
    }
}
fn scan_relation<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> Token {
    let mut buf = String::new();
    while let Some(&c) = it.peek() {
        match c {
            '>' | '<' | '=' | '!' => buf.push(c),
            _ => break,
        }
        it.next();
    }
    match buf.as_str() {
        "==" => Token::Eq,
        "!=" => Token::Ne,
        "<" => Token::Lt,
        "<=" => Token::Lte,
        ">" => Token::Gt,
        ">=" => Token::Gte,
        "!" => Token::Not,
        _ => Token::BadRelation(buf),
    }
}

fn scan_text<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> Token {
    let mut buf = String::new();
    let mut escape = false;
    let mut open = false;
    let quote = it.peek().unwrap().to_owned();
    while let Some(ch) = it.next() {
        if ch == '\\' && !escape {
            escape = true;
        } else if !open && ch == quote {
            open = true;
        } else if open && ch == quote && !escape {
            open = false;
            break;
        } else {
            buf.push(ch);
            escape = false
        }
    }
    match open {
        true => Token::UnclosedText(buf),
        false => Token::Text(buf),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_quoted_escaped_string() {
        let test = r#""\"Escaped\" \\\a \ \"\"\"""#;
        let want = Token::Text(r#""Escaped" \a  """"#.to_owned());
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            None => panic!("no token"),
        }
    }
    #[test]
    fn test_single_quoted_escaped_string() {
        let test = r#"'a \'text\''  sdf sdfjasdfksad jasdfças d"#;
        let want = Token::Text(r#"a 'text'"#.to_owned());
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_unclosed_string() {
        match scan(r#""Start a string but never finish it"#).get(0) {
            Some(t) => assert_eq!(
                *t,
                Token::UnclosedText(r#"Start a string but never finish it"#.to_owned())
            ),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_integral_number() {
        let test = r#"43223212"#;
        let want = Token::Number(43223212.0);
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_decimal_number() {
        let test = r#"432232.12"#;
        let want = Token::Number(432232.12);
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_negative_number() {
        let test = r#"-43223212"#;
        let want = Token::Number(-43223212.0);
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_wrong_negative_number() {
        let test = r#"--43223212"#;
        let want = Token::Illegal('-');
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_ws() {
        let test = r#"       anything"#;
        let want = Token::Ws;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_eof() {
        let test = r#""#;
        let want = Token::Eof;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_comma() {
        let test = r#", anything after"#;
        let want = Token::Comma;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_eq_as_symbol() {
        let test = r#"==5"#;
        let want = Token::Eq;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_eq_insensitive() {
        let test = r#"eQ 5"#;
        let want = Token::Eq;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_ne_as_symbol() {
        let test = r#"!=5"#;
        let want = Token::Ne;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_ne_insensitive() {
        let test = r#"nE 5"#;
        let want = Token::Ne;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_lt_as_symbol() {
        let test = r#"<5"#;
        let want = Token::Lt;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_lt_insensitive() {
        let test = r#"lT 5"#;
        let want = Token::Lt;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_lte_as_symbol() {
        let test = r#"<=5"#;
        let want = Token::Lte;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_lte_insensitive() {
        let test = r#"lTe 5"#;
        let want = Token::Lte;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_gt_as_symbol() {
        let test = r#">5"#;
        let want = Token::Gt;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_gt_insensitive() {
        let test = r#"gT 5"#;
        let want = Token::Gt;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_gte_as_symbol() {
        let test = r#">=5"#;
        let want = Token::Gte;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_gte_insensitive() {
        let test = r#"gTe 5"#;
        let want = Token::Gte;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_not_as_symbol_with_space() {
        let test = r#"! something to negate"#;
        let want = Token::Not;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_not_as_symbol_without_space() {
        let test = r#"!something to negate"#;
        let want = Token::Not;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_not_insensitive() {
        let test = r#"NoT Something to negate"#;
        let want = Token::Not;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_null_insensitive() {
        let test = r#"NuLl Something to negate"#;
        let want = Token::None;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_nil_insensitive() {
        let test = r#"NiL Something to negate"#;
        let want = Token::None;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_nothing_insensitive() {
        let test = r#"NoThInG Something to negate"#;
        let want = Token::None;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_defined_insensitive() {
        let test = r#"DefiNed Something to negate"#;
        let want = Token::Defined;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_true_insensitive() {
        let test = r#"TruE ..."#;
        let want = Token::True;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_yes_insensitive() {
        let test = r#"YeS ..."#;
        let want = Token::True;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_false_insensitive() {
        let test = r#"FalsE ..."#;
        let want = Token::False;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_in_insensitive() {
        let test = r#"In ..."#;
        let want = Token::In;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_in_as_at_symbol() {
        let test = r#"@ALL ..."#;
        let want = Token::In;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_in_as_hashtag_symbol() {
        let test = r#"#ALL ..."#;
        let want = Token::In;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_all_insensitive() {
        let test = r#"AlL[4,3,2]"#;
        let want = Token::All;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_any_insensitive() {
        let test = r#"AnY[4,3,2]"#;
        let want = Token::Any;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_and_insensitive() {
        let test = r#"AnD [4,3,2]"#;
        let want = Token::And;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_and_with_symbol() {
        let test = r#"&&[4,3,2]"#;
        let want = Token::And;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_or_insensitive() {
        let test = r#"Or [4,3,2]"#;
        let want = Token::Or;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_or_with_symbol() {
        let test = r#"||[4,3,2]"#;
        let want = Token::Or;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_lt_bracket() {
        let test = r#"[4,3,2]"#;
        let want = Token::LtBracket;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_rt_bracket() {
        let test = r#"] sda"#;
        let want = Token::RtBracket;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_lt_parenthesis() {
        let test = r#"(4,3,2)"#;
        let want = Token::LtParentheses;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_rt_parenthesis() {
        let test = r#") sda"#;
        let want = Token::RtParentheses;
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }

    #[test]
    fn test_ident_dotted() {
        let test = r#".field.name sdf"#;
        let want = Token::Ident(r#".field.name"#.to_owned());
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
    #[test]
    fn test_ident() {
        let test = r#"field.name sdf"#;
        let want = Token::Ident(r#"field.name"#.to_owned());
        match scan(test).get(0) {
            Some(t) => assert_eq!(*t, want),
            _ => panic!("no token"),
        }
    }
}
