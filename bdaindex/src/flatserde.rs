use serde_json::Value;
use std::collections::{HashMap, VecDeque};

pub struct FlatJsonValueIterator {
    stack: VecDeque<(Vec<String>, Value)>,
}
impl FlatJsonValueIterator {
    pub fn new(v: &Value) -> Self {
        FlatJsonValueIterator {
            stack: VecDeque::from([(Vec::new(), v.clone())]),
        }
    }
}
impl Iterator for FlatJsonValueIterator {
    type Item = (String, Value);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop_front() {
                Some(fv) => match fv {
                    (f, Value::Array(vs)) => {
                        for v in vs {
                            self.stack.push_back((f.clone(), v))
                        }
                    }
                    (f, Value::Object(vs)) => {
                        for (ff, v) in vs {
                            let mut f = f.clone();
                            f.push(ff);
                            self.stack.push_back((f, v))
                        }
                    }
                    (f, v) => return Some((format!(".{}", f.join(".")), v)),
                },
                None => return None,
            }
        }
    }
}

pub struct FlatJsonFieldIterator {
    stack: VecDeque<(Vec<String>, Value)>,
    field: VecDeque<String>,
    visit: HashMap<String, bool>,
}

impl FlatJsonFieldIterator {
    pub fn new(v: &Value) -> Self {
        FlatJsonFieldIterator {
            stack: VecDeque::from([(Vec::new(), v.clone())]),
            field: VecDeque::new(),
            visit: HashMap::new(),
        }
    }
}

impl Iterator for FlatJsonFieldIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop_front() {
                Some((f, v)) => {
                    let k = format!(".{}", f.join("."));
                    if !self.visit.contains_key(&k) {
                        self.visit.insert(k.clone(), true);
                        self.field.push_back(k)
                    }
                    match v {
                        Value::Array(vs) => {
                            for v in vs {
                                self.stack.push_back((f.clone(), v))
                            }
                        }
                        Value::Object(vs) => {
                            for (ff, v) in vs {
                                let mut f = f.clone();
                                f.push(ff);
                                self.stack.push_back((f, v))
                            }
                        }
                        _ => {}
                    }
                    if !self.field.is_empty() {
                        return self.field.pop_front();
                    }
                }
                None => return self.field.pop_front(),
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_value_iterator() {
        let v = json!(
        {
            "keya":"vala",
            "keyb":
                [
                    "valb1",
                    "valb2",
                    "valb3"
                ],
            "keyc":
                {
                    "keyca": 1 as i64,
                    "keycb": ["a","b"],
                    "keycc": 2 as i64,
                    "keycd": ["c","d"]
                }
        });
        let mut iter = FlatJsonValueIterator::new(&v);
        assert_eq!(iter.next(), Some((".keya".to_string(), json!("vala"))));
        assert_eq!(iter.next(), Some((".keyb".to_string(), json!("valb1"))));
        assert_eq!(iter.next(), Some((".keyb".to_string(), json!("valb2"))));
        assert_eq!(iter.next(), Some((".keyb".to_string(), json!("valb3"))));
        assert_eq!(
            iter.next(),
            Some((".keyc.keyca".to_string(), json!(1 as i64)))
        );
        assert_eq!(
            iter.next(),
            Some((".keyc.keycc".to_string(), json!(2 as i64)))
        );
        assert_eq!(iter.next(), Some((".keyc.keycb".to_string(), json!("a"))));
        assert_eq!(iter.next(), Some((".keyc.keycb".to_string(), json!("b"))));
        assert_eq!(iter.next(), Some((".keyc.keycd".to_string(), json!("c"))));
        assert_eq!(iter.next(), Some((".keyc.keycd".to_string(), json!("d"))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_field_iterator() {
        let v = json!(
        {
            "keya":"vala",
            "keyb":
                [
                    "valb1",
                    "valb2",
                    "valb3"
                ],
            "keyc":
                {
                    "keyca": 1 as i64,
                    "keycb": ["a","b"],
                    "keycc": 2 as i64,
                    "keycd": ["c","d"]
                }
        });
        let mut iter = FlatJsonFieldIterator::new(&v);
        assert_eq!(iter.next(), Some(".".to_string()));
        assert_eq!(iter.next(), Some(".keya".to_string()));
        assert_eq!(iter.next(), Some(".keyb".to_string()));
        assert_eq!(iter.next(), Some(".keyc".to_string()));
        assert_eq!(iter.next(), Some(".keyc.keyca".to_string()));
        assert_eq!(iter.next(), Some(".keyc.keycb".to_string()));
        assert_eq!(iter.next(), Some(".keyc.keycc".to_string()));
        assert_eq!(iter.next(), Some(".keyc.keycd".to_string()));
    }
}
