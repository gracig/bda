pub mod backend;
pub mod bql;
pub mod flatserde;
use backend::{Batch, IndexKey, IndexValue};
use bql::{Value, BQL};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    error::Error,
    fmt::Debug,
    ops::{Bound, RangeBounds},
    sync::Arc,
};

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
        ast: Box<BQL>,
    ) -> Result<Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>, Box<dyn Error>> {
        match *ast {
            BQL::And(a, b) => Ok(self.and(self.search(a)?, self.search(b)?)),
            BQL::Or(a, b) => Ok(self.or(self.search(a)?, self.search(b)?)),
            BQL::Diff(a, b) => Ok(self.diff(self.search(a)?, self.search(b)?)),
            BQL::Comp(a, b) => Ok(self.complement(self.search(a)?, self.search(b)?)),
            BQL::IsPresent => self.all(),
            BQL::Eq { field, value } => self.is_eq(&field, &value),
            BQL::IsDefined { field } => self.is_defined(&field),
            BQL::LT { field, value } => self.is_less(&field, &value),
            BQL::LE { field, value } => self.is_less_or_eq(&field, &value),
            BQL::GT { field, value } => self.is_greater(&field, &value),
            BQL::GE { field, value } => self.is_greater_or_eq(&field, &value),
            BQL::All { field, values } => self.contains_all(&field, &values),
            BQL::Any { field, values } => self.contains_any(&field, &values),
            BQL::Not(b) => match *b {
                BQL::And(..)
                | BQL::Or(..)
                | BQL::Diff(..)
                | BQL::Comp(..)
                | BQL::Not(..)
                | BQL::IsPresent => Ok(self.diff(self.all()?, self.search(b)?)),
                BQL::Eq { field: ref f, .. }
                | BQL::IsDefined { field: ref f, .. }
                | BQL::LT { field: ref f, .. }
                | BQL::LE { field: ref f, .. }
                | BQL::GT { field: ref f, .. }
                | BQL::GE { field: ref f, .. }
                | BQL::All { field: ref f, .. }
                | BQL::Any { field: ref f, .. } => {
                    Ok(self.diff(self.is_defined(f)?, self.search(b)?))
                }
            },
        }
    }

    pub fn field_values(
        &self,
        field: &str,
    ) -> Result<Box<dyn Iterator<Item = Result<Value, Box<dyn Error>>>>, Box<dyn Error>> {
        self.backend
            .key_scan(min_key(field)..max_key(field))
            .and_then(|iter| {
                Ok(Box::new(iter.filter_map(|ks| match ks {
                    Ok(key) => match key {
                        IndexKey::FieldKey { field: _ } => None,
                        IndexKey::ValueKey { field: _, value } => Some(Ok(value)),
                    },
                    Err(e) => Some(Err(e)),
                }))
                    as Box<dyn Iterator<Item = Result<Value, Box<dyn Error>>>>)
            })
    }

    pub fn is_defined(
        &self,
        field: &str,
    ) -> Result<Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>, Box<dyn Error>> {
        self.backend.value_scan(
            &IndexKey::FieldKey {
                field: field.to_owned(),
            },
            ..,
        )
    }

    pub fn all(
        &self,
    ) -> Result<Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>, Box<dyn Error>> {
        self.is_defined(".")
    }

    pub fn is_less(
        &self,
        field: &str,
        value: &Value,
    ) -> Result<Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>, Box<dyn Error>> {
        self.range(min_key(field)..vkey(field, value), true)
    }

    pub fn is_less_or_eq(
        &self,
        field: &str,
        value: &Value,
    ) -> Result<Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>, Box<dyn Error>> {
        self.range(min_key(field)..=vkey(field, value), true)
    }

    pub fn is_greater(
        &self,
        field: &str,
        value: &Value,
    ) -> Result<Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>, Box<dyn Error>> {
        self.range(vkey(field, value)..max_key(field), true)
    }

    pub fn is_greater_or_eq(
        &self,
        field: &str,
        value: &Value,
    ) -> Result<Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>, Box<dyn Error>> {
        self.range(vkey(field, value)..max_key(field), false)
    }

    pub fn is_eq(
        &self,
        field: &str,
        value: &Value,
    ) -> Result<Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>, Box<dyn Error>> {
        self.range(vkey(field, value)..=vkey(field, value), false)
    }

    pub fn contains_all(
        &self,
        field: &str,
        values: &Vec<Value>,
    ) -> Result<Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>, Box<dyn Error>> {
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
    ) -> Result<Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>, Box<dyn Error>> {
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
        a: Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>,
        b: Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>,
    ) -> Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>> {
        Box::new(IndexValueMerge::new(SetOperation::And, a, b))
    }

    pub fn or(
        &self,
        a: Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>,
        b: Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>,
    ) -> Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>> {
        Box::new(IndexValueMerge::new(SetOperation::Or, a, b))
    }
    pub fn diff(
        &self,
        a: Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>,
        b: Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>,
    ) -> Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>> {
        Box::new(IndexValueMerge::new(SetOperation::Diff, a, b))
    }
    pub fn complement(
        &self,
        a: Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>,
        b: Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>,
    ) -> Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>> {
        Box::new(IndexValueMerge::new(SetOperation::Diff, b, a))
    }

    fn range<R: RangeBounds<IndexKey> + Clone + 'static>(
        &self,
        range: R,
        exclude_start: bool,
    ) -> Result<Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>, Box<dyn Error>> {
        self.backend.key_scan(range.clone()).and_then(|ks_iter| {
            ks_iter
                .filter(|item| match item {
                    Ok(item) => {
                        if exclude_start {
                            if let Bound::Included(x) = range.clone().start_bound() {
                                item.ne(x)
                            } else {
                                true
                            }
                        } else {
                            true
                        }
                    }
                    Err(_) => true,
                })
                .try_fold(Vec::new(), |mut stack, key| {
                    key.and_then(|ref key| {
                        self.backend.value_scan(key, ..).and_then(|vs_iter| {
                            stack.push(vs_iter);
                            if stack.len() > 1 {
                                let a = stack.pop().unwrap();
                                let b = stack.pop().unwrap();
                                stack.push(self.or(a, b));
                            }
                            Ok(stack)
                        })
                    })
                })
                .and_then(|mut stack| Ok(stack.pop().unwrap_or(Box::new(Vec::new().into_iter()))))
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
    iter_a: Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>,
    iter_b: Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>,
    read_a: bool,
    read_b: bool,
    next_a: Option<IndexValue>,
    next_b: Option<IndexValue>,
    set_op: SetOperation,
}

impl IndexValueMerge {
    fn new(
        set_op: SetOperation,
        iter_a: Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>,
        iter_b: Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>,
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
    type Item = Result<IndexValue, Box<dyn Error>>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.read_a {
                let a = self.iter_a.next();
                match a {
                    Some(Ok(x)) => self.next_a = Some(x),
                    Some(Err(e)) => {
                        self.next_a = None;
                        self.next_b = None;
                        self.read_a = false;
                        self.read_b = false;
                        return Some(Err(e));
                    }
                    None => {
                        self.read_a = false;
                        self.next_a = None;
                    }
                }
            }
            if self.read_b {
                let b = self.iter_b.next();
                match b {
                    Some(Ok(x)) => self.next_b = Some(x),
                    Some(Err(e)) => {
                        self.next_a = None;
                        self.next_b = None;
                        self.read_a = false;
                        self.read_b = false;
                        return Some(Err(e));
                    }
                    None => {
                        self.read_b = false;
                        self.next_b = None;
                    }
                }
            }
            match (&self.next_a, &self.next_b) {
                (None, None) => return None,
                (None, Some(b)) => {
                    self.read_a = false;
                    self.read_b = true;
                    match self.set_op {
                        SetOperation::And | SetOperation::Diff => {}
                        SetOperation::Or => return Some(Ok(b.clone())),
                    }
                }
                (Some(a), None) => {
                    self.read_a = true;
                    self.read_b = false;
                    match self.set_op {
                        SetOperation::And => {}
                        SetOperation::Or | SetOperation::Diff => return Some(Ok(a.clone())),
                    }
                }
                (Some(a), Some(b)) if a < b => {
                    self.read_a = true;
                    self.read_b = false;
                    match self.set_op {
                        SetOperation::And => {}
                        SetOperation::Or | SetOperation::Diff => return Some(Ok(a.clone())),
                    }
                }
                (Some(a), Some(b)) if a > b => {
                    self.read_a = false;
                    self.read_b = true;
                    match self.set_op {
                        SetOperation::And | SetOperation::Diff => {}
                        SetOperation::Or => return Some(Ok(b.clone())),
                    }
                }
                (Some(a), Some(_)) => {
                    self.read_a = true;
                    self.read_b = true;
                    match self.set_op {
                        SetOperation::Diff => {}
                        SetOperation::And | SetOperation::Or => return Some(Ok(a.clone())),
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
