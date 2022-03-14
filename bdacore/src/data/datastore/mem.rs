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
    fn get<'a>(&self, id: &'a EntityID) -> Result<Option<Entity>, Box<dyn Error>> {
        Ok(self.kvstore.get(id))
    }
    fn set(&self, action: Op) -> Result<Op, Box<dyn Error>> {
        match action {
            Op::Create { ref new } => {
                match new {
                    Entity::Resource(id, r) => {
                        self.kvstore.put(new);
                        match id {
                            EntityID::ResourceID(id) => self.index.insert(id, r.clone()),
                        }
                    }
                }?;
            }
            Op::Update { ref new, ref old } => {
                match old {
                    Entity::Resource(id, r) => match id {
                        EntityID::ResourceID(id) => self.index.remove(id, r.clone()),
                    },
                }?;
                match new {
                    Entity::Resource(id, r) => {
                        self.kvstore.put(new);
                        match id {
                            EntityID::ResourceID(id) => self.index.insert(id, r.clone()),
                        }
                    }
                }?;
            }
            Op::Delete { ref id, ref old } => {
                match old {
                    Entity::Resource(id, r) => match id {
                        EntityID::ResourceID(id) => self.index.remove(id, r.clone()),
                    },
                }?;
                self.kvstore.del(id);
            }
        };
        Ok(action)
    }
    fn search<'a>(&self, query: &'a Query) -> Result<EntityIDIterator, Box<dyn Error>> {
        self.index
            .search(Box::new(query.ast.clone()))
            .and_then(|iter| {
                Ok(iter.filter_map(|rv| match rv {
                    Ok(v) => match v {
                        bdaindex::backend::IndexValue::IDStrValue(id) => {
                            Some(Ok(EntityID::ResourceID(id)))
                        }
                        bdaindex::backend::IndexValue::IDIntValue(_) => None,
                    },
                    Err(e) => Some(Err(e)),
                }))
            })
            .and_then(|x| Ok(Box::new(x) as EntityIDIterator))
    }

    fn values<'a>(
        &self,
        _kind: &'a EntityKind,
        field: &'a str,
    ) -> Result<ValueIterator, Box<dyn Error>> {
        self.index.field_values(field)
    }
}
