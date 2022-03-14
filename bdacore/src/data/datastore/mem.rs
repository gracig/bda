use bdaindex::{backend::Backend, Index};

pub mod kvstore;

use crate::data::*;

pub struct MemDatastore<T: Backend> {
    kvstore: kvstore::KvStore,
    index: Index<T>,
}

pub fn new<T: Backend>(index_backend: Arc<T>) -> MemDatastore<T> {
    MemDatastore::new(index_backend)
}

impl<T: Backend> MemDatastore<T> {
    pub fn new(index_backend: Arc<T>) -> Self {
        MemDatastore {
            kvstore: kvstore::new(),
            index: Index::new(index_backend),
        }
    }
}

impl<T: Backend> Datastore for MemDatastore<T> {
    fn get<'a>(&self, id: &'a EntityID) -> Result<Option<Entity>, String> {
        Ok(self.kvstore.get(id))
    }
    fn set(&self, action: Op) -> Result<Op, String> {
        match action {
            Op::Create { ref new } => {
                match new {
                    Entity::Resource(id, r) => {
                        self.kvstore.put(new);
                        match id {
                            EntityID::ResourceID(id) => self
                                .index
                                .insert(id, r.clone())
                                .map_err(|e| e.to_string())?,
                        }
                    }
                };
            }
            Op::Update { ref new, ref old } => {
                match old {
                    Entity::Resource(id, r) => match id {
                        EntityID::ResourceID(id) => self
                            .index
                            .remove(id, r.clone())
                            .map_err(|e| e.to_string())?,
                    },
                };
                match new {
                    Entity::Resource(id, r) => {
                        self.kvstore.put(new);
                        match id {
                            EntityID::ResourceID(id) => self
                                .index
                                .insert(id, r.clone())
                                .map_err(|e| e.to_string())?,
                        }
                    }
                };
            }
            Op::Delete { ref id, ref old } => {
                match old {
                    Entity::Resource(id, r) => match id {
                        EntityID::ResourceID(id) => self
                            .index
                            .remove(id, r.clone())
                            .map_err(|e| e.to_string())?,
                    },
                };
                self.kvstore.del(id);
            }
        };
        Ok(action)
    }
    fn search<'a>(&self, query: &'a Query) -> Result<Box<dyn Iterator<Item = EntityID>>, String> {
        self.index
            .search(Box::new(query.ast.clone()))
            .and_then(|iter| {
                Ok(iter.filter_map(|v| match v {
                    bdaindex::backend::IndexValue::IDStrValue(id) => Some(EntityID::ResourceID(id)),
                    bdaindex::backend::IndexValue::IDIntValue(_) => None,
                }))
            })
            .and_then(|x| Ok(Box::new(x) as Box<dyn Iterator<Item = EntityID>>))
            .map_err(|e| e.to_string())
    }

    fn values<'a>(
        &self,
        _kind: &'a EntityKind,
        field: &'a str,
    ) -> Result<Box<dyn Iterator<Item = Value>>, String> {
        self.index.field_values(field).map_err(|e| e.to_string())
    }
}
