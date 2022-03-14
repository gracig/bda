use super::{Backend, IndexKey, IndexValue, IndexValueIterator, KeyScanIterator};
use core::slice;
use ffi::{MDB_val, MDB_GET_BOTH, MDB_NEXT_DUP, MDB_NEXT_NODUP, MDB_SET_RANGE};
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
                    flags.insert(DatabaseFlags::DUP_SORT);
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
                    Ok(_) => Ok(()),
                    Err(e) => match e {
                        lmdb::Error::KeyExist => Ok(()),
                        _ => Err(e),
                    },
                },
                super::BatchOp::Del(k, v) => {
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
            .and_then(|_| tx.commit())
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn key_scan<R: RangeBounds<super::IndexKey> + 'static>(
        &self,
        range: R,
    ) -> Result<KeyScanIterator, Box<dyn Error>> {
        Ok(Box::new(KeyScanIter::new(self.env.clone(), self.lmdb, range)?) as KeyScanIterator)
    }

    fn value_scan<R: RangeBounds<IndexValue> + 'static>(
        &self,
        key: &IndexKey,
        range: R,
    ) -> Result<IndexValueIterator, Box<dyn Error>> {
        Ok(
            Box::new(ValueScanIter::new(self.env.clone(), self.lmdb, key, range)?)
                as IndexValueIterator,
        )
    }
}

struct ValueScanIter {
    env: Arc<Environment>,
    db: Database,
    key: Vec<u8>,
    next_val: Option<Option<IndexValue>>,
    last_val: Option<IndexValue>,
    last_include: bool,
}
impl ValueScanIter {
    fn new<R: RangeBounds<IndexValue>>(
        env: Arc<Environment>,
        db: Database,
        key: &IndexKey,
        range: R,
    ) -> Result<Self, Box<dyn Error>> {
        let (next_val, last_val, last_include) = match range.start_bound() {
            Bound::Included(start) | Bound::Excluded(start) => match range.end_bound() {
                Bound::Included(end) => (Some(Some(start.to_owned())), Some(end.to_owned()), true),
                Bound::Excluded(end) => (Some(Some(start.to_owned())), Some(end.to_owned()), false),
                Bound::Unbounded => (Some(Some(start.to_owned())), None, false),
            },
            Bound::Unbounded => match range.end_bound() {
                Bound::Included(end) => (Some(None), Some(end.to_owned()), true),
                Bound::Excluded(end) => (Some(None), Some(end.to_owned()), false),
                Bound::Unbounded => (Some(None), None, false),
            },
        };
        Ok(ValueScanIter {
            env,
            db,
            key: key.serialize()?,
            next_val,
            last_val,
            last_include,
        })
    }
}

//implemented in imperative way
impl Iterator for ValueScanIter {
    type Item = Result<IndexValue, Box<dyn Error>>;
    fn next(&mut self) -> Option<Self::Item> {
        let next_val = match self.next_val.clone() {
            Some(x) => x,
            None => return None,
        };
        let b_val = match next_val.as_ref() {
            Some(v) => match v.serialize() {
                Ok(bv) => Some(bv),
                Err(e) => return Some(Err(e)),
            },
            None => None,
        };
        let tx = match self.env.begin_ro_txn() {
            Ok(tx) => tx,
            Err(e) => return Some(Err(Box::new(e) as Box<dyn Error>)),
        };
        let cursor = match tx.open_ro_cursor(self.db) {
            Ok(cursor) => cursor,
            Err(e) => return Some(Err(Box::new(e) as Box<dyn Error>)),
        };
        let op = if let None = next_val {
            MDB_SET_RANGE
        } else {
            MDB_GET_BOTH
        };
        let actual = match cursor.get(Some(&self.key), b_val.as_ref().map(|v| v as &[u8]), op) {
            Ok((_, v)) => match IndexValue::deserialize(v) {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            },
            Err(e) => match e {
                lmdb::Error::NotFound => return None,
                _ => return Some(Err(Box::new(e) as Box<dyn Error>)),
            },
        };
        let ref b_actual = match actual.serialize() {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        self.next_val = match cursor.get(Some(&self.key), Some(b_actual as &[u8]), MDB_NEXT_DUP) {
            Ok((_, v)) => match IndexValue::deserialize(v) {
                Ok(v) => Some(Some(v)),
                Err(e) => return Some(Err(e)),
            },
            Err(e) => match e {
                lmdb::Error::NotFound => None,
                _ => return Some(Err(Box::new(e) as Box<dyn Error>)),
            },
        };
        match self.last_val {
            Some(ref last_val) => {
                if actual.lt(last_val) || actual.eq(last_val) && self.last_include {
                    Some(Ok(actual))
                } else {
                    self.next_val = None;
                    None
                }
            }
            None => Some(Ok(actual)),
        }
    }
}

struct KeyScanIter {
    env: Arc<Environment>,
    db: Database,
    next_key: Option<IndexKey>,
    last_key: IndexKey,
    last_include: bool,
}
impl KeyScanIter {
    fn new<R: RangeBounds<IndexKey>>(
        env: Arc<Environment>,
        db: Database,
        range: R,
    ) -> Result<Self, Box<dyn Error>> {
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
        Ok(KeyScanIter {
            env,
            db,
            next_key: Some(start),
            last_key,
            last_include,
        })
    }
}

//implemented in functional way
impl Iterator for KeyScanIter {
    type Item = Result<IndexKey, Box<dyn Error>>;
    fn next(&mut self) -> Option<Self::Item> {
        let next_key = self.next_key.clone();
        next_key.and_then(|next_key| {
            next_key.serialize().map_or_else(
                |e| Some(Err(e)),
                |l_key| {
                    self.env.begin_rw_txn().map_or_else(
                        |e| Some(Err(Box::new(e) as Box<dyn Error>)),
                        |tx| {
                            tx.open_ro_cursor(self.db).map_or_else(
                                |e| Some(Err(Box::new(e) as Box<dyn Error>)),
                                |cursor| match cursor.get(Some(&l_key), None, MDB_SET_RANGE) {
                                    Ok((Some(k), _)) => IndexKey::deserialize(k).map_or_else(
                                        |e| Some(Err(e)),
                                        |key| {
                                            if key.lt(&self.last_key)
                                                || key.eq(&self.last_key) && self.last_include
                                            {
                                                match cursor.get(Some(k), None, MDB_NEXT_NODUP) {
                                                    Ok((Some(k), _)) => IndexKey::deserialize(k)
                                                        .map_or_else(
                                                            |e| Some(Err(e)),
                                                            |nkey| {
                                                                self.next_key = Some(nkey);
                                                                Some(Ok(key))
                                                            },
                                                        ),
                                                    _ => {
                                                        self.next_key = None;
                                                        Some(Ok(key))
                                                    }
                                                }
                                            } else {
                                                self.next_key = None;
                                                None
                                            }
                                        },
                                    ),
                                    _ => {
                                        self.next_key = None;
                                        None
                                    }
                                },
                            )
                        },
                    )
                },
            )
        })
    }
}
