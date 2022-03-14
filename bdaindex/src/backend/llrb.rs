use std::error::Error;

use crate::backend::{Backend, Batch, IndexKey, IndexValue};
use ppom::mdb::OMap;

use super::KeyScanItem;

pub struct LLRBBackend {
    llrb: OMap<IndexKey, OMap<IndexValue, bool>>,
}

impl LLRBBackend {
    pub fn new() -> Self {
        LLRBBackend { llrb: OMap::new() }
    }
}

impl Backend for LLRBBackend {
    fn update(&self, batch: Batch) -> Result<(), Box<dyn std::error::Error>> {
        batch.iter().try_for_each(|op| match op {
            super::BatchOp::Add(f, v) => self
                .llrb
                .get(&f)
                .or_else(|_| Ok((OMap::new(), 0 as u64)))
                .and_then(|(tree, _)| {
                    tree.set(v, true)?;
                    self.llrb.set(f, tree)?;
                    Ok(())
                }),
            super::BatchOp::Del(f, v) => self
                .llrb
                .get(&f)
                .map_err(|e| Box::new(e) as Box<dyn Error>)
                .and_then(|(id_set, _)| {
                    id_set.remove(&v)?;
                    if id_set.is_empty() {
                        self.llrb.remove(&f)?;
                    } else {
                        self.llrb.set(f, id_set)?;
                    }
                    Ok(())
                }),
        })
    }

    fn key_scan<R: std::ops::RangeBounds<IndexKey> + 'static>(
        &self,
        range: R,
    ) -> Result<Box<dyn Iterator<Item = super::KeyScanItem>>, Box<dyn Error>> {
        self.llrb
            .range(range)
            .map_err(|e| Box::new(e) as Box<dyn Error>)
            .and_then(|r| {
                Ok(Box::new(r.into_iter().map(|(k, v)| {
                    KeyScanItem {
                        key: k,
                        min: v
                            .iter()
                            .and_then(|iter| Ok(iter.min().map(|(k, _)| k)))
                            .ok()
                            .unwrap()
                            .unwrap(),
                        max: v
                            .iter()
                            .and_then(|iter| Ok(iter.max().map(|(k, _)| k)))
                            .ok()
                            .unwrap()
                            .unwrap(),
                    }
                })) as Box<dyn Iterator<Item = KeyScanItem>>)
            })
    }

    fn value_scan<R: std::ops::RangeBounds<IndexValue> + 'static>(
        &self,
        key: &IndexKey,
        range: R,
    ) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>> {
        self.llrb
            .get(key)
            .and_then(|(values, _)| values.range(range))
            .and_then(|items| {
                Ok(Box::new(items.map(|(v, _)| v)) as Box<dyn Iterator<Item = IndexValue>>)
            })
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

#[cfg(test)]
mod test_super {

    use super::*;
    use crate::backend::BatchOp;
    use crate::bql::{Rational, Value};
    use serde_json::json;

    #[test]
    fn test_update() {
        //Test BatchOp::Add
        let add_batch_1 = Batch::add_data(
            "id1",
            json!({
                "keya":true,
                "keyb":["valb1",],
                "keyc": {
                    "keyca": 1 as i64,
                    "keycb": ["a","b"],
                    "keycc": 2 as i64, "keycd": ["c","d"]
                }
            }),
        )
        .unwrap();
        let add_batch_2 = Batch::add_data(
            "id2",
            json!({
                "2keya":true,
                "2keyb":["2valb1",],
            }),
        )
        .unwrap();
        let backend = LLRBBackend::new();
        assert_eq!(backend.llrb.is_empty(), true);
        backend.update(add_batch_1.clone()).unwrap();
        assert_eq!(backend.llrb.len(), add_batch_1.items.len());
        backend.update(add_batch_2.clone()).unwrap();
        assert_eq!(
            backend.llrb.len(),
            add_batch_1.items.len() + add_batch_2.items.len() - 1 // the field . is common
        );
        //Test BatchOp::Del
        let del_batch_1 = Batch {
            items: add_batch_1
                .iter()
                .map(|op| {
                    if let BatchOp::Add(k, v) = op {
                        BatchOp::Del(k, v)
                    } else {
                        panic!("only BatchOp::Add are permitted");
                    }
                })
                .collect(),
        };
        let del_batch_2 = Batch {
            items: add_batch_2
                .iter()
                .map(|op| {
                    if let BatchOp::Add(k, v) = op {
                        BatchOp::Del(k, v)
                    } else {
                        panic!("only BatchOp::Add are permitted");
                    }
                })
                .collect(),
        };
        assert_eq!(
            backend.llrb.len(),
            del_batch_1.items.len() + del_batch_2.items.len() - 1 // the field . is common
        );
        backend.update(del_batch_1.clone()).unwrap();
        assert_eq!(backend.llrb.len(), del_batch_2.items.len());
        backend.update(del_batch_2.clone()).unwrap();
        assert_eq!(backend.llrb.is_empty(), true);
    }

    #[test]
    fn test_keyscan() {
        //Test keyscan
        let add_batch_1 = Batch::add_data(
            "id1",
            json!({
                "keya":true,
                "keyb":["valb1",],
                "keyc": {
                    "keyca": 1 as i64,
                    "keycb": ["a","b"],
                    "keycc": 2 as i64,
                    "keycd": ["c","d"]
                }
            }),
        )
        .unwrap();
        let add_batch_2 = Batch::add_data(
            "id2",
            json!({
                "keya":true,
                "keyb":["valb1",],
            }),
        )
        .unwrap();
        let add_batch_3 = Batch::add_data(
            "id3",
            json!({
                "keya":true,
                "keyb":["valb1","valb2"],
            }),
        )
        .unwrap();
        let backend = LLRBBackend::new();
        backend.update(add_batch_1).unwrap();
        backend.update(add_batch_2).unwrap();
        backend.update(add_batch_3).unwrap();

        let ikey = |k: &str| IndexKey::FieldKey {
            field: k.to_owned(),
        };
        let vkey = |k: &str, v: Value| IndexKey::ValueKey {
            field: k.to_owned(),
            value: v,
        };
        let ival = |v: &str| IndexValue::IDStrValue(v.to_owned());
        let fskey = |k: &str, min: Option<&'static str>, max: Option<&'static str>| KeyScanItem {
            key: ikey(k),
            min: min.map(|v| ival(v)).unwrap(),
            max: max.map(|v| ival(v)).unwrap(),
        };
        let vskey =
            |k: &str, v: Value, min: Option<&'static str>, max: Option<&'static str>| KeyScanItem {
                key: vkey(k, v),
                min: min.map(|v| ival(v)).unwrap(),
                max: max.map(|v| ival(v)).unwrap(),
            };

        //index value scan
        let range = vkey(".keyb", Value::Bottom)..vkey(".keyb", Value::Top);
        let mut scan = backend.key_scan(range).unwrap();
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyb",
                Value::Text("valb1".to_owned()),
                Some("id1"),
                Some("id3")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyb",
                Value::Text("valb2".to_owned()),
                Some("id3"),
                Some("id3")
            ))
        );
        assert_eq!(scan.next(), None);

        //inclusive range. (like equal)
        let range = ikey(".keya")..=ikey(".keya");
        let mut scan = backend.key_scan(range).unwrap();
        assert_eq!(scan.next(), Some(fskey(".keya", Some("id1"), Some("id3"))));
        assert_eq!(scan.next(), None);

        //Field index scan range. (only field indexes)
        let range = ..vkey("", Value::Bottom);
        let mut scan = backend.key_scan(range).unwrap();
        assert_eq!(scan.next(), Some(fskey(".", Some("id1"), Some("id3"))));
        assert_eq!(scan.next(), Some(fskey(".keya", Some("id1"), Some("id3"))));
        assert_eq!(scan.next(), Some(fskey(".keyb", Some("id1"), Some("id3"))));
        assert_eq!(scan.next(), Some(fskey(".keyc", Some("id1"), Some("id1"))));
        assert_eq!(
            scan.next(),
            Some(fskey(".keyc.keyca", Some("id1"), Some("id1")))
        );
        assert_eq!(
            scan.next(),
            Some(fskey(".keyc.keycb", Some("id1"), Some("id1")))
        );
        assert_eq!(
            scan.next(),
            Some(fskey(".keyc.keycc", Some("id1"), Some("id1")))
        );
        assert_eq!(
            scan.next(),
            Some(fskey(".keyc.keycd", Some("id1"), Some("id1")))
        );
        assert_eq!(scan.next(), None);

        //Value index scan range. (only value indexes)
        let range = vkey("", Value::Bottom)..;
        let mut scan = backend.key_scan(range).unwrap();
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keya",
                Value::Boolean(true),
                Some("id1"),
                Some("id3")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyb",
                Value::Text("valb1".to_owned()),
                Some("id1"),
                Some("id3")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyb",
                Value::Text("valb2".to_owned()),
                Some("id3"),
                Some("id3")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyc.keyca",
                Value::Rational(Rational::from(1.0)),
                Some("id1"),
                Some("id1")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyc.keycb",
                Value::Text("a".to_owned()),
                Some("id1"),
                Some("id1")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyc.keycb",
                Value::Text("b".to_owned()),
                Some("id1"),
                Some("id1")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyc.keycc",
                Value::Rational(Rational::from(2.0)),
                Some("id1"),
                Some("id1")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyc.keycd",
                Value::Text("c".to_owned()),
                Some("id1"),
                Some("id1")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyc.keycd",
                Value::Text("d".to_owned()),
                Some("id1"),
                Some("id1")
            ))
        );
        assert_eq!(scan.next(), None);

        //Full scan range. (everybody, including values)
        let range = ..;
        let mut scan = backend.key_scan(range).unwrap();
        assert_eq!(scan.next(), Some(fskey(".", Some("id1"), Some("id3"))));
        assert_eq!(scan.next(), Some(fskey(".keya", Some("id1"), Some("id3"))));
        assert_eq!(scan.next(), Some(fskey(".keyb", Some("id1"), Some("id3"))));
        assert_eq!(scan.next(), Some(fskey(".keyc", Some("id1"), Some("id1"))));
        assert_eq!(
            scan.next(),
            Some(fskey(".keyc.keyca", Some("id1"), Some("id1")))
        );
        assert_eq!(
            scan.next(),
            Some(fskey(".keyc.keycb", Some("id1"), Some("id1")))
        );
        assert_eq!(
            scan.next(),
            Some(fskey(".keyc.keycc", Some("id1"), Some("id1")))
        );
        assert_eq!(
            scan.next(),
            Some(fskey(".keyc.keycd", Some("id1"), Some("id1")))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keya",
                Value::Boolean(true),
                Some("id1"),
                Some("id3")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyb",
                Value::Text("valb1".to_owned()),
                Some("id1"),
                Some("id3")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyb",
                Value::Text("valb2".to_owned()),
                Some("id3"),
                Some("id3")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyc.keyca",
                Value::Rational(Rational::from(1.0)),
                Some("id1"),
                Some("id1")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyc.keycb",
                Value::Text("a".to_owned()),
                Some("id1"),
                Some("id1")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyc.keycb",
                Value::Text("b".to_owned()),
                Some("id1"),
                Some("id1")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyc.keycc",
                Value::Rational(Rational::from(2.0)),
                Some("id1"),
                Some("id1")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyc.keycd",
                Value::Text("c".to_owned()),
                Some("id1"),
                Some("id1")
            ))
        );
        assert_eq!(
            scan.next(),
            Some(vskey(
                ".keyc.keycd",
                Value::Text("d".to_owned()),
                Some("id1"),
                Some("id1")
            ))
        );

        assert_eq!(scan.next(), None);
    }
}
