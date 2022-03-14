pub mod index;
pub mod kvstore;

use crate::model::data::*;

pub struct MemDatastore {
    kvstore: kvstore::KvStore,
    index: index::Index,
}

pub fn new() -> MemDatastore {
    MemDatastore::new()
}

impl MemDatastore {
    pub fn new() -> Self {
        MemDatastore {
            kvstore: kvstore::new(),
            index: index::new(),
        }
    }
}

impl Datastore for MemDatastore {
    fn get<'a>(&self, id: &'a EntityID) -> Result<Option<Entity>, String> {
        Ok(self.kvstore.get(id))
    }
    fn set(&self, action: Op) -> Result<Op, String> {
        match action {
            Op::Create { ref new } => {
                self.kvstore.put(new);
                self.index.index(new)?;
            }
            Op::Update { ref new, ref old } => {
                self.kvstore.put(new);
                self.index.remove(old)?;
                self.index.index(new)?;
            }
            Op::Delete { ref id, ref old } => {
                self.index.remove(old)?;
                self.kvstore.del(id);
            }
        }
        Ok(action)
    }
    fn search<'a>(&self, query: &'a Query) -> Result<Box<dyn Iterator<Item = EntityID>>, String> {
        Ok(self.index.search(&query.kind, Box::new(query.ast.clone())))
    }
}
