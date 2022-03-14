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
    pub fn put(&self, entity: &Entity) -> Result<(), String> {
        self.db
            .set(entity.id(), entity.clone())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn del(&self, id: &EntityID) -> Result<(), String> {
        self.db.remove(id).map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn get(&self, id: &EntityID) -> Option<Entity> {
        self.db.get(id).ok().and_then(|(e, _)| Some(e))
    }
}
