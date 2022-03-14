use super::{Backend, IndexKey, IndexValue, KeyScanItem};
use core::slice;
use ffi::{MDB_val, MDB_GET_BOTH, MDB_NEXT, MDB_SET_RANGE};
use lmdb::{
    Cursor, Database, DatabaseFlags, Environment, EnvironmentFlags, Transaction, WriteFlags,
};
use lmdb_sys as ffi;
use std::{
    error::Error,
    ops::{Bound, RangeBounds},
    path::Path,
    sync::Arc,
};

pub struct LMDBBackend {
    env: Arc<Environment>,
    lmdb: Database,
}

impl LMDBBackend {
    pub fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        Ok(Environment::new()).and_then(|mut builder| {
            builder.set_flags(EnvironmentFlags::empty());
            builder
                .open(path)
                .and_then(|env| {
                    let mut flags = DatabaseFlags::empty();
                    flags.insert(DatabaseFlags::DUP_SORT); //dup_sort alows multiple values by key
                    let db = env.create_db(None, flags).unwrap();
                    unsafe {
                        let txn = env.begin_rw_txn()?;
                        ffi::mdb_set_compare(
                            txn.txn(),
                            db.dbi(),
                            cmp_index as *mut ffi::MDB_cmp_func,
                        );
                        ffi::mdb_set_dupsort(
                            txn.txn(),
                            db.dbi(),
                            cmp_value as *mut ffi::MDB_cmp_func,
                        );
                        txn.commit()?;
                    }
                    Ok(LMDBBackend {
                        env: Arc::new(env),
                        lmdb: db,
                    })
                })
                .map_err(|e| Box::new(e) as Box<dyn Error>)
        })
    }
}
/*
unsafe fn slice_to_val(slice: Option<&[u8]>) -> ffi::MDB_val {
    match slice {
        Some(slice) => ffi::MDB_val {
            mv_size: slice.len() as size_t,
            mv_data: slice.as_ptr() as *mut c_void,
        },
        None => ffi::MDB_val {
            mv_size: 0,
            mv_data: ptr::null_mut(),
        },
    }
}
*/
#[no_mangle]
extern "C" fn cmp_index(a: *const MDB_val, b: *const MDB_val) -> i32 {
    let x: Option<IndexKey>;
    let y: Option<IndexKey>;
    unsafe {
        x = IndexKey::deserialize(slice::from_raw_parts(
            (*a).mv_data as *const u8,
            (*a).mv_size as usize,
        ))
        .ok();
        y = IndexKey::deserialize(slice::from_raw_parts(
            (*b).mv_data as *const u8,
            (*b).mv_size as usize,
        ))
        .ok();
    }
    x.and_then(|x| y.and_then(|y| Some(x.cmp(&y) as i32)))
        .unwrap_or(0)
}
#[no_mangle]
extern "C" fn cmp_value(a: *const MDB_val, b: *const MDB_val) -> i32 {
    let x: Option<IndexValue>;
    let y: Option<IndexValue>;
    unsafe {
        x = IndexValue::deserialize(slice::from_raw_parts(
            (*a).mv_data as *const u8,
            (*a).mv_size as usize,
        ))
        .ok();
        y = IndexValue::deserialize(slice::from_raw_parts(
            (*b).mv_data as *const u8,
            (*b).mv_size as usize,
        ))
        .ok();
    }
    x.and_then(|x| y.and_then(|y| Some(x.cmp(&y) as i32)))
        .unwrap_or(0)
}

impl Backend for LMDBBackend {
    fn update(&self, batch: super::Batch) -> Result<(), Box<dyn std::error::Error>> {
        let mut tx = self.env.begin_rw_txn()?;
        batch
            .iter()
            .try_for_each(|op| match op {
                super::BatchOp::Add(k, v) => match tx.put(
                    self.lmdb,
                    &bincode::serialize(&k).unwrap(),
                    &bincode::serialize(&v).unwrap(),
                    WriteFlags::NO_DUP_DATA,
                ) {
                    Ok(_) => {
                        println!("Adding k {:?} v {:?}", k, v);
                        Ok(())
                    }
                    Err(e) => match e {
                        lmdb::Error::KeyExist => {
                            println!("Ignoring k {:?} v {:?}", k, v);
                            Ok(())
                        }
                        _ => Err(e),
                    },
                },
                super::BatchOp::Del(k, v) => {
                    println!("Deleting k {:?} v {:?}", k, v);
                    let mut cursor = tx.open_rw_cursor(self.lmdb)?;
                    match cursor.get(
                        Some(&bincode::serialize(&k).unwrap()),
                        Some(&bincode::serialize(&v).unwrap()),
                        MDB_GET_BOTH,
                    ) {
                        Ok(_) => cursor.del(WriteFlags::empty()),
                        Err(e) => match e {
                            lmdb::Error::NotFound => Ok(()),
                            _ => Err(e),
                        },
                    }
                }
            })
            .and_then(|_| {
                println!("txcommit");
                tx.commit()
            })
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn key_scan<R: RangeBounds<super::IndexKey> + 'static>(
        &self,
        range: R,
    ) -> Result<Box<dyn Iterator<Item = super::KeyScanItem>>, Box<dyn Error>> {
        println!(
            "scanning bounds start:{:?} end: {:?}",
            range.start_bound(),
            range.end_bound()
        );
        let (start, last_key, last_include) = match range.start_bound() {
            Bound::Included(start) | Bound::Excluded(start) => match range.end_bound() {
                Bound::Included(end) => (start.to_owned(), end.to_owned(), true),
                Bound::Excluded(end) => (start.to_owned(), end.to_owned(), false),
                Bound::Unbounded => (start.to_owned(), IndexKey::top(), false),
            },
            Bound::Unbounded => match range.end_bound() {
                Bound::Included(end) => (IndexKey::bottom(), end.to_owned(), true),
                Bound::Excluded(end) => (IndexKey::bottom(), end.to_owned(), false),
                Bound::Unbounded => (IndexKey::bottom(), IndexKey::top(), false),
            },
        };
        Ok(Box::new(KeyScanIter::new(
            self.env.clone(),
            self.lmdb,
            start,
            last_key,
            last_include,
        )?) as Box<dyn Iterator<Item = KeyScanItem>>)
    }

    fn value_scan<R: RangeBounds<IndexValue> + 'static>(
        &self,
        _key: &IndexKey,
        _range: R,
    ) -> Result<Box<dyn Iterator<Item = IndexValue>>, Box<dyn Error>> {
        todo!()
    }
}

struct KeyScanIter {
    env: Arc<Environment>,
    db: Database,
    next_key: Option<IndexKey>,
    next_val: Option<IndexValue>,
    next_min: Option<IndexValue>,
    last_key: IndexKey,
    last_include: bool,
}
impl KeyScanIter {
    fn new(
        env: Arc<Environment>,
        db: Database,
        start: IndexKey,
        last_key: IndexKey,
        last_include: bool,
    ) -> Result<Self, Box<dyn Error>> {
        println!(
            "Building iterator start: {:?} last: {:?} include: {:?}",
            start, last_key, last_include
        );
        Ok(KeyScanIter {
            env,
            db,
            next_key: Some(start),
            next_val: None,
            next_min: None,
            last_key,
            last_include,
        })
    }
}
impl Iterator for KeyScanIter {
    type Item = KeyScanItem;
    fn next(&mut self) -> Option<Self::Item> {
        if let None = self.next_key {
            return None;
        }
        let tx = self.env.begin_rw_txn().unwrap();
        let cursor = tx.open_ro_cursor(self.db).unwrap();
        let mut flag = MDB_SET_RANGE;
        loop {
            let current = (
                self.next_key.clone().unwrap(),
                self.next_val.clone(),
                self.next_min.clone(),
            );
            let lookup = (
                Some(current.0.serialize()),
                current.1.as_ref().map(|v| v.serialize()),
            );
            match cursor.get(
                lookup.0.as_ref().map(|v| v as &[u8]),
                lookup.1.as_ref().map(|v| v as &[u8]),
                flag,
            ) {
                Err(e) => {
                    self.next_key = None;
                    if let lmdb::Error::NotFound = e {
                        let item = KeyScanItem {
                            key: current.0,
                            min: current.2.unwrap(),
                            max: current.1.unwrap(),
                        };
                        if item.key.lt(&self.last_key)
                            || (item.key.eq(&self.last_key) && self.last_include)
                        {
                            self.next_min = self.next_val.clone();
                            println!("Sending {:?}", item);
                            return Some(item);
                        } else {
                            return None;
                        }
                    }
                    eprintln!("LMDB Error {:?} {:?}", e, current);
                }
                Ok((None, _)) => {
                    println!("not found {:?}", lookup);
                    self.next_key = None;
                    return None;
                }
                Ok((Some(k), v)) => {
                    flag = MDB_NEXT;
                    self.next_key = Some(IndexKey::deserialize(k).unwrap());
                    self.next_val = Some(IndexValue::deserialize(v).unwrap());
                    if let None = current.1 {
                        self.next_min = self.next_val.clone();
                        continue;
                    }
                    if self.next_key.as_ref().unwrap().eq(&current.0) {
                        continue;
                    }
                    let item = KeyScanItem {
                        key: current.0,
                        min: current.2.unwrap(),
                        max: current.1.unwrap(),
                    };
                    if item.key.lt(&self.last_key)
                        || (item.key.eq(&self.last_key) && self.last_include)
                    {
                        self.next_min = self.next_val.clone();
                        println!("Sending {:?}", item);
                        return Some(item);
                    } else {
                        self.next_key = None;
                        println!("Finish iterator");
                        return None;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;
    use crate::{
        backend::{Batch, BatchOp},
        bql::{Rational, Value},
    };
    use serde_json::json;
    use tempdir::TempDir;
    #[test]
    fn test_updatea() {
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
        let tmp_dir = TempDir::new("/tmp/lmdb").unwrap();
        let path = tmp_dir.path();
        let backend = LMDBBackend::new(path).unwrap();
        assert_eq!(backend.env.stat().unwrap().entries(), 0);
        println!("Start adding batch 1");
        backend.update(add_batch_1.clone()).unwrap();
        println!("Start adding batch 2");
        backend.update(add_batch_2.clone()).unwrap();
        println!("Start removing batch 1");
        backend.update(del_batch_1.clone()).unwrap();
        println!("Start removing batch 2");
        backend.update(del_batch_2.clone()).unwrap();
        assert_eq!(backend.env.stat().unwrap().entries(), 0);
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
        let tmp_dir = TempDir::new("/tmp/lmdb").unwrap();
        let path = tmp_dir.path();
        let backend = LMDBBackend::new(path).unwrap();
        assert_eq!(backend.env.stat().unwrap().entries(), 0);
        println!("Start adding batch 1");
        backend.update(add_batch_1.clone()).unwrap();
        println!("Start adding batch 2");
        backend.update(add_batch_2.clone()).unwrap();
        println!("Start adding batch 3");
        backend.update(add_batch_3.clone()).unwrap();

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
        assert_eq!(scan.next(), None);
    }
}
