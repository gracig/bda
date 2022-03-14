use crate::model::data::{Entity, EntityID};
use ppom::mdb::OMap;

pub struct KvStore {
    db: OMap<EntityID, Entity>,
}

pub fn new() -> KvStore {
    KvStore::new()
}

impl KvStore {
    pub fn new() -> Self {
        KvStore { db: OMap::new() }
    }
    pub fn put(&self, entity: &Entity) -> Option<Entity> {
        self.db.set(entity.id(), entity.clone()).ok()?
    }
    pub fn del(&self, id: &EntityID) -> Option<Entity> {
        self.db.remove(id).ok()?
    }
    pub fn get(&self, id: &EntityID) -> Option<Entity> {
        Some(self.db.get(id).ok()?.0)
    }
}
