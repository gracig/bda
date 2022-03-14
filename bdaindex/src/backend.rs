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

type KeyScanIterator = Box<dyn Iterator<Item = Result<IndexKey, Box<dyn Error>>>>;
type IndexValueIterator = Box<dyn Iterator<Item = Result<IndexValue, Box<dyn Error>>>>;

#[cfg_attr(test, automock)]
pub trait Backend {
    fn update(&self, batch: Batch) -> Result<(), Box<dyn std::error::Error>>;

    fn key_scan<R: RangeBounds<IndexKey> + 'static>(
        &self,
        range: R,
    ) -> Result<KeyScanIterator, Box<dyn Error>>;

    fn value_scan<R: RangeBounds<IndexValue> + 'static>(
        &self,
        key: &IndexKey,
        range: R,
    ) -> Result<IndexValueIterator, Box<dyn Error>>;
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
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize, Hash)]
pub enum IndexKey {
    FieldKey { field: String },
    ValueKey { field: String, value: BValue },
}
impl IndexKey {
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        bincode::serialize(&self).map_err(|e| Box::new(e) as Box<dyn Error>)
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
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        bincode::serialize(&self).map_err(|e| Box::new(e) as Box<dyn Error>)
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

    pub fn append(&mut self, other: &mut Batch) {
        self.items.append(&mut other.items);
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
                                value: BValue::from_json(v),
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

#[cfg(test)]
mod test_super {
    use std::{collections::HashMap, ops::RangeBounds};

    use serde_json::json;
    use tempdir::TempDir;

    use super::{llrb::LLRBBackend, lmdb::LMDBBackend, Backend, IndexValue};
    use crate::{
        backend::{Batch, BatchOp, IndexKey},
        bql::Value,
        flatserde::{FlatJsonFieldIterator, FlatJsonValueIterator},
    };

    #[test]
    fn test_lmdb_backend() {
        let tmp_dir = TempDir::new("/tmp/lmdb").unwrap();
        let path = tmp_dir.path();
        let backend = LMDBBackend::new(path).unwrap();
        test_backend("lmdb", backend);
    }
    #[test]
    fn test_llrb_backend() {
        test_backend("llrb", LLRBBackend::new());
    }

    struct TestData {
        keys: Vec<IndexKey>,
        values: HashMap<IndexKey, Vec<IndexValue>>,
        batch_add: Batch,
        batch_del: Batch,
    }
    impl TestData {
        fn from(values: Vec<(&str, serde_json::Value)>) -> Self {
            let mut keys: Vec<IndexKey> = Vec::new();
            let mut batch_add = Batch { items: Vec::new() };
            let mut values_map: HashMap<IndexKey, Vec<IndexValue>> = HashMap::new();
            for ref v in values {
                keys.append(
                    &mut FlatJsonFieldIterator::new(&v.1)
                        .map(|field| IndexKey::FieldKey { field })
                        .chain(FlatJsonValueIterator::new(&v.1).map(|(field, value)| {
                            let id = IndexValue::IDStrValue(v.0.to_string());
                            let key = IndexKey::ValueKey {
                                field,
                                value: Value::from_json(value),
                            };
                            match values_map.get_mut(&key) {
                                Some(v) => {
                                    //Not optimal.. but this is for test data. it simulates ordered values
                                    if !v.contains(&id) {
                                        v.push(id);
                                        v.sort();
                                        v.dedup();
                                    }
                                }
                                None => {
                                    values_map.insert(key.clone(), vec![id]);
                                }
                            }
                            key
                        }))
                        .collect(),
                );
                batch_add.append(&mut Batch::add_data(v.0, v.1.clone()).unwrap());
            }
            keys.sort();
            keys.dedup();

            TestData {
                keys,
                values: values_map,
                batch_del: Batch {
                    items: batch_add
                        .items
                        .iter()
                        .map(|x| {
                            if let BatchOp::Add(k, v) = x {
                                BatchOp::Del(k.clone(), v.clone())
                            } else {
                                x.clone()
                            }
                        })
                        .collect(),
                },
                batch_add,
            }
        }
    }

    fn test_backend<T: Backend>(name: &str, backend: T) {
        assert_eq!(backend.key_scan(..).unwrap().count(), 0);
        let test_data = TestData::from(vec![
            (
                "id1",
                json!({
                    "zzz":true,
                    "keya":true,
                    "keyb":["valb1",],
                    "keyc": {
                        "keyca": 1 as i64,
                        "keycb": ["a","b"],
                        "keycc": 2 as i64,
                        "keycd": ["c","d", "e", "f"]
                    }
                }),
            ),
            (
                "id2",
                json!({
                    "zzz":true,
                    "keya":true,
                    "keyb":["valb1",],
                    "keyc": {
                        "keyca": 1 as i64,
                        "keycb": ["a","b"],
                        "keycc": 2 as i64,
                        "keycd": ["c","d", "e", "f"]
                    }
                }),
            ),
            (
                "id3",
                json!({
                    "zzz":true,
                    "keya":true,
                    "keyb":["valb1","valb2"],
                    "keyc": {
                        "keyca": 1 as i64,
                        "keycb": ["a","b"],
                        "keycc": 2 as i64,
                        "keycd": ["c","d", "e", "f"]
                    }
                }),
            ),
        ]);
        backend.update(test_data.batch_add.clone()).unwrap();
        let size = backend.key_scan(..).unwrap().count();
        assert!(size > 0);
        backend.update(test_data.batch_add.clone()).unwrap(); //checking idempotence and dedup
        backend.update(test_data.batch_add.clone()).unwrap(); //checking idempotence and dedup
        assert!(backend.key_scan(..).unwrap().count() == size);

        //test key_scan : checking full range
        println!("Testing backend {} full range", name);
        test_backend_keyscan(name, &backend, &test_data, ..);

        //test key_scan : checking one item inclusive end range
        println!("Testing backend {} one item inclusive range", name);
        test_backend_keyscan(
            name,
            &backend,
            &test_data,
            IndexKey::FieldKey {
                field: ".zzz".to_owned(),
            }..=IndexKey::FieldKey {
                field: ".zzz".to_owned(),
            },
        );

        //test key_scan : checking one item exclusive end range
        println!("Testing backend {} one item exclusive range", name);
        test_backend_keyscan(
            name,
            &backend,
            &test_data,
            IndexKey::FieldKey {
                field: ".zzz".to_owned(),
            }..IndexKey::FieldKey {
                field: ".zzz".to_owned(),
            },
        );

        //value_scan
        test_backend_valuescan(
            name,
            &backend,
            &IndexKey::ValueKey {
                field: ".zzz".to_owned(),
                value: Value::Boolean(true),
            },
            &test_data,
            ..,
        );

        //deleting keys
        println!("Testing backend {} delete keys", name);
        backend.update(test_data.batch_del.clone()).unwrap();
        assert_eq!(backend.key_scan(..).unwrap().count(), 0);
        backend.update(test_data.batch_del.clone()).unwrap(); //checking idempotence and dedup
        backend.update(test_data.batch_del.clone()).unwrap(); //checking idempotence and dedup
        assert_eq!(backend.key_scan(..).unwrap().count(), 0);
    }

    fn test_backend_valuescan<T: Backend, R: RangeBounds<IndexValue> + Clone + 'static>(
        name: &str,
        backend: &T,
        key: &IndexKey,
        test_data: &TestData,
        range: R,
    ) {
        backend
            .value_scan(key, range.clone())
            .and_then(|mut value_scan_iter| {
                let ids = test_data.values.get(key).unwrap();
                for expected in ids.into_iter().filter(|v| range.contains(v)) {
                    println!(
                        "backend {}. checking key {:?}, value {:?}",
                        name, key, expected
                    );
                    let got = value_scan_iter.next().unwrap().unwrap();
                    assert_eq!(
                        *expected, got,
                        "backend {}. expected value {:?} but got value {:?}",
                        name, expected, got
                    );
                }
                if let Some(x) = value_scan_iter.next() {
                    assert!(false, "Expecting none but got {:?}", x)
                }
                if let Some(x) = value_scan_iter.next() {
                    assert!(false, "Expecting none but got {:?}", x)
                }
                Ok(())
            })
            .unwrap();
    }

    fn test_backend_keyscan<T: Backend, R: RangeBounds<IndexKey> + Clone + 'static>(
        name: &str,
        backend: &T,
        test_data: &TestData,
        range: R,
    ) {
        backend
            .key_scan(range.clone())
            .and_then(|mut key_scan_iter| {
                for expected in test_data
                    .keys
                    .clone()
                    .into_iter()
                    .filter(|x| range.contains(x))
                {
                    println!("backend {}. checking key {:?}", name, expected);
                    let got = key_scan_iter.next().unwrap().unwrap();
                    assert_eq!(
                        expected, got,
                        "backend {}. expected key {:?} but got key {:?}",
                        name, expected, got
                    );
                }
                if let Some(x) = key_scan_iter.next() {
                    assert!(false, "Expecting none but got {:?}", x)
                }
                Ok(())
            })
            .unwrap();
    }
}
