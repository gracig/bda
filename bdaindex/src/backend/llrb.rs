use std::error::Error;

use crate::backend::{Backend, Batch, IndexKey, IndexValue};
use ppom::mdb::OMap;

use super::{IndexValueIterator, KeyScanIterator};

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
                .or_else(|_| Ok((OMap::new(), 0 as u64)))
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
    ) -> Result<KeyScanIterator, Box<dyn Error>> {
        self.llrb
            .range(range)
            .map_err(|e| Box::new(e) as Box<dyn Error>)
            .and_then(|r| Ok(Box::new(r.into_iter().map(|(k, _)| Ok(k))) as KeyScanIterator))
    }

    fn value_scan<R: std::ops::RangeBounds<IndexValue> + 'static>(
        &self,
        key: &IndexKey,
        range: R,
    ) -> Result<IndexValueIterator, Box<dyn Error>> {
        self.llrb
            .get(key)
            .or_else(|_| Ok((OMap::new(), 0 as u64)))
            .and_then(|(values, _)| values.range(range))
            .and_then(|items| Ok(Box::new(items.map(|(v, _)| Ok(v))) as IndexValueIterator))
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}
