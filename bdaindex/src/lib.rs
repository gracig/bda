use bdaql::{Rational, Value as BValue};
use flat::{FlatFieldIterator, FlatValueIterator};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value as JValue;
use std::{error::Error, fmt::Debug, sync::Arc};

mod backend {
    #[cfg(test)]
    use mockall::{automock, predicate::*};
    #[cfg_attr(test, automock)]
    pub trait Backend {
        fn update(&self, batch: super::Batch) -> Result<(), Box<dyn std::error::Error>>;
    }
}

pub struct Index<T: backend::Backend> {
    name: String,
    backend: Arc<T>,
}
pub fn new<T: backend::Backend>(name: &str, backend: Arc<T>) -> Index<T> {
    Index::new(name, backend)
}

impl<T: backend::Backend> Index<T> {
    pub fn new(name: &str, backend: Arc<T>) -> Self {
        Self {
            name: name.to_string(),
            backend,
        }
    }
    pub fn index<V>(&self, id: &str, data: V) -> Result<(), Box<dyn Error>>
    where
        V: Clone + Debug + PartialEq<V> + Serialize + Eq + DeserializeOwned,
    {
        make_batch(&self.name, id, data).and_then(|batch| self.backend.update(batch))
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize)]
pub enum IndexKey {
    FieldKey {
        index: String,
        field: String,
    },
    ValueKey {
        index: String,
        field: String,
        value: Option<BValue>,
    },
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize)]
pub enum IndexValue {
    IDStrValue(String),
    IDIntValue(usize),
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize)]
pub enum BatchOp {
    Add(IndexKey, IndexValue),
    Put(IndexKey, IndexValue),
    Del(IndexKey),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Batch {
    items: Vec<BatchOp>,
}
impl Batch {
    pub fn new() -> Self {
        Batch { items: Vec::new() }
    }
    pub fn iter(&self) -> BatchIter {
        BatchIter {
            iter: Box::new(self.items.clone().into_iter()),
        }
    }
}
pub struct BatchIter {
    iter: Box<dyn Iterator<Item = BatchOp>>,
}
impl Iterator for BatchIter {
    type Item = BatchOp;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
fn to_bql_value(jv: JValue) -> Option<BValue> {
    match jv {
        JValue::Bool(vv) => Some(BValue::Boolean(vv)),
        JValue::Number(vv) => match vv.as_f64() {
            Some(n) => Some(BValue::Rational(Rational::from(n))),
            None => Some(BValue::Rational(Rational::from(f64::NAN))),
        },
        JValue::String(vv) => Some(BValue::Text(vv)),
        _ => None,
    }
}

fn make_batch<V>(index_name: &str, id: &str, data: V) -> Result<Batch, Box<dyn Error>>
where
    V: Clone + Debug + PartialEq<V> + Serialize + Eq + DeserializeOwned,
{
    serde_json::to_value(&data)
        .map_err(|error| Box::new(error) as Box<dyn std::error::Error>)
        .and_then(|ref data| {
            Ok(Batch {
                items: FlatValueIterator::new(data)
                    .map(|(k, v)| {
                        BatchOp::Add(
                            IndexKey::ValueKey {
                                index: index_name.to_owned(),
                                field: k.to_owned(),
                                value: to_bql_value(serde_json::to_value(v).unwrap()),
                            },
                            IndexValue::IDStrValue(id.to_owned()),
                        )
                    })
                    .chain(FlatFieldIterator::new(data).map(|k| {
                        BatchOp::Add(
                            IndexKey::FieldKey {
                                index: index_name.to_owned(),
                                field: k.to_owned(),
                            },
                            IndexValue::IDStrValue(id.to_owned()),
                        )
                    }))
                    .collect(),
            })
        })
}

#[cfg(test)]
mod test_super {

    use super::*;
    use mockall::predicate;
    use serde_json::json;

    #[test]
    fn test_index() {
        let data = json!({
            "keya":true,
            "keyb":["valb1",],
            "keyc": {
                "keyca": 1 as i64,
                "keycb": ["a","b"],
                "keycc": 2 as i64, "keycd": ["c","d"]
            }
        });
        let ns = "my_index";
        let id = "my_id";
        let batch = make_batch(ns.clone(), id.clone(), data.clone()).unwrap();
        let mut backend = backend::MockBackend::new();
        backend
            .expect_update()
            .times(1)
            .with(predicate::eq(batch.clone()))
            .returning(|_| Ok(()));
        let index = Index::new(ns.clone(), Arc::new(backend));
        match index.index(id.clone(), data.clone()) {
            Ok(_) => {}
            Err(e) => panic!("Unexpected error on test: {}", e),
        }

        let mut backend = backend::MockBackend::new();
        backend
            .expect_update()
            .times(1)
            .with(predicate::eq(batch.clone()))
            .returning(|_| Err("new error")?);
        let index = Index::new(ns.clone(), Arc::new(backend));
        match index.index(id, data) {
            Ok(_) => panic!("Should have returned an error"),
            Err(_) => {}
        }
    }
}

mod flat {
    use serde_json::Value;
    use std::collections::{HashMap, VecDeque};

    pub struct FlatValueIterator {
        stack: VecDeque<(Vec<String>, Value)>,
    }
    impl FlatValueIterator {
        pub fn new(v: &Value) -> Self {
            FlatValueIterator {
                stack: VecDeque::from([(Vec::new(), v.clone())]),
            }
        }
    }
    impl Iterator for FlatValueIterator {
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

    pub struct FlatFieldIterator {
        stack: VecDeque<(Vec<String>, Value)>,
        field: VecDeque<String>,
        visit: HashMap<String, bool>,
    }

    impl FlatFieldIterator {
        pub fn new(v: &Value) -> Self {
            FlatFieldIterator {
                stack: VecDeque::from([(Vec::new(), v.clone())]),
                field: VecDeque::new(),
                visit: HashMap::new(),
            }
        }
    }

    impl Iterator for FlatFieldIterator {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            while let Some((f, v)) = self.stack.pop_front() {
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
            }
            return self.field.pop_front();
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
            let mut iter = FlatValueIterator::new(&v);
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
            let mut iter = FlatFieldIterator::new(&v);
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
}
