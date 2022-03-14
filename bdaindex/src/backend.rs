pub mod llrb;
pub mod lmdb;
use crate::bql::{Rational, Value as BValue};
use crate::flatserde::{FlatJsonFieldIterator, FlatJsonValueIterator};
#[cfg(test)]
use mockall::{automock, predicate::*};
use serde::Deserialize;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value as JValue;
use std::{error::Error, fmt::Debug, ops::RangeBounds};

#[cfg_attr(test, automock)]
pub trait Backend {
    fn update(&self, batch: Batch) -> Result<(), Box<dyn std::error::Error>>;

    fn key_scan<R: RangeBounds<IndexKey> + 'static>(
        &self,
        range: R,
    ) -> Result<Box<dyn Iterator<Item = KeyScanItem>>, Box<dyn Error>>;

    fn value_scan<R: RangeBounds<IndexValue> + 'static>(
        &self,
        key: &IndexKey,
        range: R,
    ) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize)]

pub struct KeyScanItem {
    pub key: IndexKey,
    pub min: IndexValue,
    pub max: IndexValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Batch {
    items: Vec<BatchOp>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize)]
pub enum BatchOp {
    Add(IndexKey, IndexValue),
    Del(IndexKey, IndexValue),
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
pub enum IndexKey {
    FieldKey { field: String },
    ValueKey { field: String, value: BValue },
}
impl IndexKey {
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn deserialize(slice: &[u8]) -> Result<Self, Box<dyn Error>> {
        bincode::deserialize(slice).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    pub fn key_bottom(&self) -> Self {
        match self {
            IndexKey::FieldKey { field: _ } => IndexKey::bottom(),
            IndexKey::ValueKey { field, value: _ } => IndexKey::ValueKey {
                field: field.to_string(),
                value: BValue::Bottom,
            },
        }
    }
    pub fn key_top(&self) -> Self {
        match self {
            IndexKey::FieldKey { field: _ } => IndexKey::top(),
            IndexKey::ValueKey { field, value: _ } => IndexKey::ValueKey {
                field: field.to_string(),
                value: BValue::Top,
            },
        }
    }

    pub fn bottom() -> Self {
        IndexKey::FieldKey {
            field: String::new(),
        }
    }
    pub fn top() -> Self {
        IndexKey::ValueKey {
            field: String::from("~"),
            value: BValue::Top,
        }
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
pub enum IndexValue {
    IDStrValue(String),
    IDIntValue(usize),
}
impl IndexValue {
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn deserialize(slice: &[u8]) -> Result<Self, Box<dyn Error>> {
        bincode::deserialize(slice).map_err(|e| Box::new(e) as Box<dyn Error>)
    }
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
    pub fn add_data<V>(id: &str, data: V) -> Result<Self, Box<dyn Error>>
    where
        V: Clone + Debug + PartialEq<V> + Serialize + DeserializeOwned,
    {
        serde_json::to_value(&data)
            .map_err(|error| Box::new(error) as Box<dyn std::error::Error>)
            .and_then(|ref data| {
                Ok(FlatJsonValueIterator::new(data)
                    .map(|(k, v)| {
                        BatchOp::Add(
                            IndexKey::ValueKey {
                                field: k.to_owned(),
                                value: match v {
                                    JValue::Bool(vv) => BValue::Boolean(vv),
                                    JValue::Number(vv) => match vv.as_f64() {
                                        Some(n) => BValue::Rational(Rational::from(n)),
                                        None => BValue::Rational(Rational::from(f64::NAN)),
                                    },
                                    JValue::String(vv) => BValue::Text(vv),
                                    _ => BValue::Bottom,
                                },
                            },
                            IndexValue::IDStrValue(id.to_owned()),
                        )
                    })
                    .chain(FlatJsonFieldIterator::new(data).map(|k| {
                        BatchOp::Add(
                            IndexKey::FieldKey {
                                field: k.to_owned(),
                            },
                            IndexValue::IDStrValue(id.to_owned()),
                        )
                    }))
                    .collect())
                .and_then(|items| Ok(Batch { items }))
            })
    }
    pub fn del_data<V>(id: &str, data: V) -> Result<Self, Box<dyn Error>>
    where
        V: Clone + Debug + PartialEq<V> + Serialize + DeserializeOwned,
    {
        serde_json::to_value(&data)
            .map_err(|error| Box::new(error) as Box<dyn std::error::Error>)
            .and_then(|ref data| {
                Ok(FlatJsonValueIterator::new(data)
                    .map(|(k, v)| {
                        BatchOp::Del(
                            IndexKey::ValueKey {
                                field: k.to_owned(),
                                value: match v {
                                    JValue::Bool(vv) => BValue::Boolean(vv),
                                    JValue::Number(vv) => match vv.as_f64() {
                                        Some(n) => BValue::Rational(Rational::from(n)),
                                        None => BValue::Rational(Rational::from(f64::NAN)),
                                    },
                                    JValue::String(vv) => BValue::Text(vv),
                                    _ => BValue::Bottom,
                                },
                            },
                            IndexValue::IDStrValue(id.to_owned()),
                        )
                    })
                    .chain(FlatJsonFieldIterator::new(data).map(|k| {
                        BatchOp::Del(
                            IndexKey::FieldKey {
                                field: k.to_owned(),
                            },
                            IndexValue::IDStrValue(id.to_owned()),
                        )
                    }))
                    .collect())
                .and_then(|items| Ok(Batch { items }))
            })
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
