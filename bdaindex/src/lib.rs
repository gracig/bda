pub mod backend;
pub mod bql;
pub mod flatserde;
use backend::{Batch, IndexKey, IndexValue};
use bql::{Ast, Value};
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt::Debug, ops::RangeBounds, sync::Arc};

pub struct Index<T: backend::Backend> {
    backend: Arc<T>,
}

pub fn new<T: backend::Backend>(backend: Arc<T>) -> Index<T> {
    Index::new(backend)
}

impl<T: backend::Backend> Index<T> {
    pub fn new(backend: Arc<T>) -> Self {
        Self { backend }
    }

    pub fn insert<V>(&self, id: &str, data: V) -> Result<(), Box<dyn Error>>
    where
        V: Clone + Debug + PartialEq<V> + Serialize + DeserializeOwned,
    {
        Batch::add_data(id, data).and_then(|batch| self.backend.update(batch))
    }

    pub fn remove<V>(&self, id: &str, data: V) -> Result<(), Box<dyn Error>>
    where
        V: Clone + Debug + PartialEq<V> + Serialize + DeserializeOwned,
    {
        Batch::del_data(id, data).and_then(|batch| self.backend.update(batch))
    }

    pub fn search(
        &self,
        ast: Box<Ast>,
    ) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>> {
        match *ast {
            Ast::Intersection(a, b) => Ok(self.and(self.search(a)?, self.search(b)?)),
            Ast::Union(a, b) => Ok(self.or(self.search(a)?, self.search(b)?)),
            Ast::Difference(a, b) => Ok(self.diff(self.search(a)?, self.search(b)?)),
            Ast::Complement(a, b) => Ok(self.complement(self.search(a)?, self.search(b)?)),
            Ast::All => self.all(),
            Ast::Equal {
                field,
                value,
                negate,
            } => self.is_eq(&field, &value).and_then(|items| {
                if negate {
                    Ok(self.diff(self.is_defined(&field)?, items))
                } else {
                    Ok(items)
                }
            }),
            Ast::Defined { field, negate } => self.is_defined(&field).and_then(|items| {
                if negate {
                    Ok(self.diff(self.all()?, items))
                } else {
                    Ok(items)
                }
            }),
            Ast::LessThan { field, value } => self.is_less(&field, &value),
            Ast::LessThanOrEqual { field, value } => self.is_less_or_eq(&field, &value),
            Ast::GreaterThan { field, value } => self.is_greater(&field, &value),
            Ast::GreaterThanOrEqual { field, value } => self.is_greater_or_eq(&field, &value),
            Ast::ContainsAll {
                field,
                values,
                negate,
            } => self.contains_all(&field, &values).and_then(|items| {
                if negate {
                    Ok(self.diff(self.all()?, items))
                } else {
                    Ok(items)
                }
            }),
            Ast::ContainsAny {
                field,
                values,
                negate,
            } => self.contains_any(&field, &values).and_then(|items| {
                if negate {
                    Ok(self.diff(self.all()?, items))
                } else {
                    Ok(items)
                }
            }),
        }
    }

    pub fn field_values(
        &self,
        field: &str,
    ) -> Result<Box<dyn Iterator<Item = Value>>, Box<dyn Error>> {
        self.backend
            .key_scan(min_key(field)..max_key(field))
            .and_then(|iter| {
                Ok(Box::new(iter.filter_map(|ks| match ks.key {
                    IndexKey::FieldKey { field: _ } => None,
                    IndexKey::ValueKey { field: _, value } => Some(value),
                })) as Box<dyn Iterator<Item = Value>>)
            })
    }

    pub fn is_defined(
        &self,
        field: &str,
    ) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>> {
        self.backend.value_scan(
            &IndexKey::FieldKey {
                field: field.to_owned(),
            },
            ..,
        )
    }

    pub fn all(&self) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>> {
        self.is_defined(".")
    }

    pub fn is_less(
        &self,
        field: &str,
        value: &Value,
    ) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>> {
        self.range(min_key(field)..vkey(field, value), true)
    }

    pub fn is_less_or_eq(
        &self,
        field: &str,
        value: &Value,
    ) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>> {
        self.range(min_key(field)..=vkey(field, value), true)
    }

    pub fn is_greater(
        &self,
        field: &str,
        value: &Value,
    ) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>> {
        self.range(vkey(field, value)..max_key(field), true)
    }

    pub fn is_greater_or_eq(
        &self,
        field: &str,
        value: &Value,
    ) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>> {
        self.range(vkey(field, value)..max_key(field), false)
    }

    pub fn is_eq(
        &self,
        field: &str,
        value: &Value,
    ) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>> {
        self.range(vkey(field, value)..=vkey(field, value), false)
    }

    pub fn contains_all(
        &self,
        field: &str,
        values: &Vec<Value>,
    ) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>> {
        values
            .into_iter()
            .try_fold(Vec::new(), |mut stack, value| {
                self.is_eq(field, value).and_then(|vs_iter| {
                    stack.push(vs_iter);
                    if stack.len() > 1 {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(self.and(a, b));
                    }
                    Ok(stack)
                })
            })
            .and_then(|mut stack| Ok(stack.pop().unwrap_or(Box::new(Vec::new().into_iter()))))
    }

    pub fn contains_any(
        &self,
        field: &str,
        values: &Vec<Value>,
    ) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>> {
        values
            .into_iter()
            .try_fold(Vec::new(), |mut stack, value| {
                self.is_eq(field, value).and_then(|vs_iter| {
                    stack.push(vs_iter);
                    if stack.len() > 1 {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(self.or(a, b));
                    }
                    Ok(stack)
                })
            })
            .and_then(|mut stack| Ok(stack.pop().unwrap_or(Box::new(Vec::new().into_iter()))))
    }

    pub fn and(
        &self,
        a: Box<dyn Iterator<Item = IndexValue>>,
        b: Box<dyn Iterator<Item = IndexValue>>,
    ) -> Box<dyn Iterator<Item = IndexValue>> {
        Box::new(IndexValueMerge::new(SetOperation::And, a, b))
    }

    pub fn or(
        &self,
        a: Box<dyn Iterator<Item = IndexValue>>,
        b: Box<dyn Iterator<Item = IndexValue>>,
    ) -> Box<dyn Iterator<Item = IndexValue>> {
        Box::new(IndexValueMerge::new(SetOperation::Or, a, b))
    }
    pub fn diff(
        &self,
        a: Box<dyn Iterator<Item = IndexValue>>,
        b: Box<dyn Iterator<Item = IndexValue>>,
    ) -> Box<dyn Iterator<Item = IndexValue>> {
        Box::new(IndexValueMerge::new(SetOperation::Diff, a, b))
    }
    pub fn complement(
        &self,
        a: Box<dyn Iterator<Item = IndexValue>>,
        b: Box<dyn Iterator<Item = IndexValue>>,
    ) -> Box<dyn Iterator<Item = IndexValue>> {
        Box::new(IndexValueMerge::new(SetOperation::Diff, b, a))
    }

    fn range<R: RangeBounds<IndexKey> + Clone + 'static>(
        &self,
        range: R,
        exclude_start: bool,
    ) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>> {
        self.backend
            .key_scan(range.clone())
            .and_then(|ks_iter| {
                Ok(ks_iter
                    .filter(|item| {
                        if exclude_start {
                            match range.clone().start_bound() {
                                std::ops::Bound::Included(x) => item.key.gt(x),
                                _ => true,
                            }
                        } else {
                            true
                        }
                    })
                    .fold(
                        (
                            Vec::new(),
                            IndexValue::IDStrValue("".to_owned()),
                            IndexValue::IDStrValue("~".to_owned()),
                        ),
                        |(mut vec, min, max), item| {
                            (
                                {
                                    vec.push(item.key);
                                    vec
                                },
                                if item.min.gt(&min) { item.min } else { min },
                                if item.max.lt(&max) { item.max } else { max },
                            )
                        },
                    ))
            })
            .and_then(|(keys, min, max)| {
                keys.into_iter()
                    .try_fold(Vec::new(), |mut stack, ref key| {
                        self.backend
                            .value_scan(key, min.clone()..=max.clone())
                            .and_then(|vs_iter| {
                                stack.push(vs_iter);
                                if stack.len() > 1 {
                                    let a = stack.pop().unwrap();
                                    let b = stack.pop().unwrap();
                                    stack.push(self.and(a, b));
                                }
                                Ok(stack)
                            })
                    })
                    .and_then(|mut stack| {
                        Ok(stack.pop().unwrap_or(Box::new(Vec::new().into_iter())))
                    })
            })
    }
}

fn vkey(field: &str, value: &Value) -> IndexKey {
    IndexKey::ValueKey {
        field: field.to_owned(),
        value: value.clone(),
    }
}
fn min_value() -> Value {
    Value::Bottom
}
fn min_key(field: &str) -> IndexKey {
    vkey(field, &min_value())
}
fn max_value() -> Value {
    Value::Top
}
fn max_key(field: &str) -> IndexKey {
    vkey(field, &max_value())
}

enum SetOperation {
    And,
    Or,
    Diff,
}
pub struct IndexValueMerge {
    iter_a: Box<dyn Iterator<Item = IndexValue>>,
    iter_b: Box<dyn Iterator<Item = IndexValue>>,
    read_a: bool,
    read_b: bool,
    next_a: Option<IndexValue>,
    next_b: Option<IndexValue>,
    set_op: SetOperation,
}

impl IndexValueMerge {
    fn new(
        set_op: SetOperation,
        iter_a: Box<dyn Iterator<Item = IndexValue>>,
        iter_b: Box<dyn Iterator<Item = IndexValue>>,
    ) -> Self {
        IndexValueMerge {
            set_op,
            iter_a,
            iter_b,
            read_a: true,
            read_b: true,
            next_a: None,
            next_b: None,
        }
    }
}

impl Iterator for IndexValueMerge {
    type Item = IndexValue;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.read_a {
                self.next_a = self.iter_a.next();
                if self.next_a == None {
                    self.read_a = false;
                }
            }
            if self.read_b {
                self.next_b = self.iter_b.next();
                if self.next_b == None {
                    self.read_b = false;
                }
            }
            match (&self.next_a, &self.next_b) {
                (None, None) => return None,
                (None, Some(b)) => {
                    self.read_a = false;
                    self.read_b = true;
                    match self.set_op {
                        SetOperation::And | SetOperation::Diff => {}
                        SetOperation::Or => return Some(b.clone()),
                    }
                }
                (Some(a), None) => {
                    self.read_a = true;
                    self.read_b = false;
                    match self.set_op {
                        SetOperation::And => {}
                        SetOperation::Or | SetOperation::Diff => return Some(a.clone()),
                    }
                }
                (Some(a), Some(b)) if a < b => {
                    self.read_a = true;
                    self.read_b = false;
                    match self.set_op {
                        SetOperation::And => {}
                        SetOperation::Or | SetOperation::Diff => return Some(a.clone()),
                    }
                }
                (Some(a), Some(b)) if a > b => {
                    self.read_a = false;
                    self.read_b = true;
                    match self.set_op {
                        SetOperation::And | SetOperation::Diff => {}
                        SetOperation::Or => return Some(b.clone()),
                    }
                }
                (Some(a), Some(_)) => {
                    self.read_a = true;
                    self.read_b = true;
                    match self.set_op {
                        SetOperation::Diff => {}
                        SetOperation::And | SetOperation::Or => return Some(a.clone()),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;
    use mockall::predicate;
    use serde_json::json;
    #[test]
    fn test_index() {
        let id = "my_id";
        let data = json!({
            "keya":true,
            "keyb":["valb1",],
            "keyc": {
                "keyca": 1 as i64,
                "keycb": ["a","b"],
                "keycc": 2 as i64, "keycd": ["c","d"]
            }
        });
        let batch = Batch::add_data(id.clone(), data.clone()).unwrap();
        let mut backend = backend::MockBackend::new();
        backend
            .expect_update()
            .times(1)
            .with(predicate::eq(batch.clone()))
            .returning(|_| Ok(()));
        let index = Index::new(Arc::new(backend));
        match index.insert(id.clone(), data.clone()) {
            Ok(_) => {}
            Err(e) => panic!("Unexpected error on test: {}", e),
        }
        let mut backend = backend::MockBackend::new();
        backend
            .expect_update()
            .times(1)
            .with(predicate::eq(batch.clone()))
            .returning(|_| Err("new error")?);
        let index = Index::new(Arc::new(backend));
        match index.insert(id, data) {
            Ok(_) => panic!("Should have returned an error"),
            Err(_) => {}
        }
    }
}
