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
        todo!()
    }

    fn set(&self, action: Op) -> Result<Op, String> {
        todo!()
    }

    fn search<'a>(&self, query: &'a Query) -> Result<EntitySet, String> {
        todo!()
    }
}
