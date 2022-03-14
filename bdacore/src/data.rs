// Modules
pub mod datastore;
pub mod query;

// Imports
use crate::{data::query::Query, logic};
use bdaproto::Resource;
use bdaql::Value;
#[cfg(test)]
use mockall::{automock, predicate::*};
use std::{fmt::Debug, sync::Arc};

#[cfg_attr(test, automock)]
pub trait Datastore {
    fn get<'a>(&self, id: &'a EntityID) -> Result<Option<Entity>, String>;
    fn set(&self, action: Op) -> Result<Op, String>;
    fn search<'a>(&self, query: &'a Query) -> Result<Box<dyn Iterator<Item = EntityID>>, String>;
    fn values<'a>(
        &self,
        kind: &'a EntityKind,
        field: &'a str,
    ) -> Result<Box<dyn Iterator<Item = Option<Value>>>, String>;
}

pub fn new(datastore: Arc<dyn Datastore + Sync + Send>) -> Data {
    Data::new(datastore)
}

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq)]
pub enum EntityKind {
    Resource,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum EntityID {
    ResourceID(String),
}
impl EntityID {
    pub fn to_kind(&self) -> EntityKind {
        match self {
            EntityID::ResourceID(_) => EntityKind::Resource,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Entity {
    Resource(EntityID, Resource),
}
impl Entity {
    pub fn to_kind(&self) -> EntityKind {
        match self {
            Entity::Resource(_, _) => EntityKind::Resource,
        }
    }
    pub fn id(&self) -> EntityID {
        match self {
            Entity::Resource(id, _) => id.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Create { new: Entity },
    Update { new: Entity, old: Entity },
    Delete { id: EntityID, old: Entity },
}
pub struct Data {
    datastore: Arc<dyn Datastore + Sync + Send + 'static>,
}

impl Data {
    pub fn new(datastore: Arc<dyn Datastore + Sync + Send>) -> Data {
        Data { datastore }
    }

    fn get<'a>(&self, id: &'a EntityID) -> Result<Option<Entity>, String> {
        self.datastore.get(id)
    }

    pub fn del<'a>(&self, id: &'a EntityID) -> Result<Option<Op>, String> {
        match self.datastore.get(id)? {
            None => Ok(None),
            Some(old) => Ok(Some(self.datastore.set(Op::Delete {
                id: id.clone(),
                old,
            })?)),
        }
    }
    fn put<'a>(&self, new: &Entity) -> Result<Option<Op>, String> {
        match new {
            Entity::Resource(id, _) => match self.datastore.get(id)? {
                None => Ok(Some(self.datastore.set(Op::Create { new: new.clone() })?)),
                Some(old) => {
                    if *new == old {
                        Ok(None)
                    } else {
                        Ok(Some(self.datastore.set(Op::Update {
                            new: new.clone(),
                            old,
                        })?))
                    }
                }
            },
        }
    }

    pub fn get_resource<'a>(&self, id: &'a EntityID) -> Result<Option<Resource>, String> {
        self.get(id).map(|oe| {
            oe.map(|entity| match entity {
                Entity::Resource(_, r) => r,
            })
        })
    }

    pub fn put_resource<'a>(&self, r: &Resource) -> Result<Option<Op>, String> {
        let mut validated = r.to_owned();
        logic::defaults(&mut validated);
        self.put(&Entity::Resource(
            logic::resource_id(&validated)?,
            validated,
        ))
    }

    pub fn search<'a>(
        &self,
        query: &'a Query,
    ) -> Result<Box<dyn Iterator<Item = EntityID>>, String> {
        self.datastore.search(query)
    }

    pub fn ids<'a>(&self, query: &'a Query) -> Result<Vec<EntityID>, String> {
        Ok(self.search(query)?.collect())
    }

    pub fn entities<'a>(&self, query: &'a Query) -> Result<Vec<Entity>, String> {
        Ok(self
            .search(query)?
            .filter_map(|ref id| self.datastore.get(id).ok()?)
            .collect())
    }

    pub fn resources<'a>(&self, query: &'a Query) -> Result<Vec<Resource>, String> {
        match query.kind {
            EntityKind::Resource => Ok(self
                .search(query)?
                .filter_map(|ref id| self.datastore.get(id).ok()?)
                .map(|Entity::Resource(_, r)| r)
                .collect()),
        }
    }

    pub fn values<'a>(
        &self,
        kind: &'a EntityKind,
        field: &'a str,
    ) -> Result<Box<dyn Iterator<Item = Option<Value>>>, String> {
        self.datastore.values(kind, field)
    }

    pub fn values_as_string<'a>(
        &self,
        kind: &'a EntityKind,
        field: &'a str,
    ) -> Result<Box<dyn Iterator<Item = String>>, String> {
        let iter = self
            .datastore
            .values(kind, field)?
            .filter_map(|ov| match ov {
                Some(v) => match v {
                    Value::Number(vv) => Some(vv.to_string()),
                    Value::Text(vv) => Some(vv),
                    Value::Boolean(vv) => Some(vv.to_string()),
                },
                None => None,
            });
        Ok(Box::new(iter))
    }

    pub fn values_as_number<'a>(
        &self,
        kind: &'a EntityKind,
        field: &'a str,
    ) -> Result<Box<dyn Iterator<Item = f64>>, String> {
        let iter = self
            .datastore
            .values(kind, field)?
            .filter_map(|ov| match ov {
                Some(v) => match v {
                    Value::Number(vv) => Some(vv),
                    Value::Text(vv) => vv.parse::<f64>().ok(),
                    Value::Boolean(vv) => {
                        if vv {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                },
                None => None,
            });
        Ok(Box::new(iter))
    }

    pub fn values_as_bool<'a>(
        &self,
        kind: &'a EntityKind,
        field: &'a str,
    ) -> Result<Box<dyn Iterator<Item = bool>>, String> {
        let iter = self
            .datastore
            .values(kind, field)?
            .filter_map(|ov| match ov {
                Some(v) => match v {
                    Value::Number(vv) => {
                        if vv == 0.0 || f64::is_nan(vv) {
                            Some(false)
                        } else {
                            Some(true)
                        }
                    }
                    Value::Text(vv) => vv.parse::<bool>().ok(),
                    Value::Boolean(vv) => Some(vv),
                },
                None => None,
            });
        Ok(Box::new(iter))
    }
}

#[cfg(test)]
mod test_super {
    use crate::logic;

    use super::*;
    use bdaql::Ast;

    #[test]
    fn test_data_search() {
        let mut mock = MockDatastore::new();
        let q = Query {
            kind: EntityKind::Resource,
            ast: Ast::All,
        };
        let entitya = EntityID::ResourceID("a".to_owned());
        let entityb = EntityID::ResourceID("b".to_owned());
        let items = vec![entitya.clone(), entityb.clone(), entityb.clone()];
        let items2 = vec![entitya.clone(), entitya.clone(), entityb.clone()];
        let mut set = Box::new(items.clone().into_iter());
        let search_set = Box::new(items2.clone().into_iter());
        mock.expect_search()
            .with(eq(q.clone()))
            .times(1)
            .returning(move |_| Ok(Box::new(search_set.clone())));
        let data = new(Arc::new(mock));
        let mut answer = data.search(&q).unwrap();
        assert_eq!(set.next(), answer.next());
        assert_ne!(set.next(), answer.next());
        assert_eq!(set.next(), answer.next());
    }

    #[test]
    fn test_data_put_new() {
        let id = EntityID::ResourceID("an id".to_owned());
        let entity = Entity::Resource(id.clone(), logic::new_resource_function("name"));
        let op = Op::Create {
            new: entity.clone(),
        };
        let mut mock = MockDatastore::new();
        mock.expect_get()
            .with(eq(id.clone()))
            .times(1)
            .returning(|_| Ok(None));
        mock.expect_set()
            .with(eq(op.clone()))
            .times(1)
            .returning(|op| Ok(op));
        let data = new(Arc::new(mock));
        assert_eq!(Some(op), data.put(&entity).unwrap());
    }
    #[test]
    fn test_data_put_same() {
        let id = EntityID::ResourceID("an id".to_owned());
        let entity = Entity::Resource(id.clone(), logic::new_resource_function("name"));
        let get_entity = entity.clone();
        let get_return = Some(get_entity);
        let mut mock = MockDatastore::new();
        mock.expect_get()
            .with(eq(id.clone()))
            .times(1)
            .returning(move |_| Ok(get_return.clone()));
        mock.expect_set().times(0);
        let data = new(Arc::new(mock));
        assert_eq!(None, data.put(&entity).unwrap());
    }
    #[test]
    fn test_data_put_change() {
        let id = EntityID::ResourceID("an id".to_owned());
        let entity = Entity::Resource(id.clone(), logic::new_resource_function("name"));
        let mut get_entity = entity.clone();
        let Entity::Resource(_, ref mut r) = get_entity;
        r.description = "last description".to_owned();
        let get_return = Some(get_entity.clone());
        let op = Op::Update {
            new: entity.clone(),
            old: get_entity.clone(),
        };
        let mut mock = MockDatastore::new();
        mock.expect_get()
            .with(eq(id.clone()))
            .times(1)
            .returning(move |_| Ok(get_return.clone()));
        mock.expect_set()
            .with(eq(op.clone()))
            .times(1)
            .returning(|op| Ok(op));
        let data = new(Arc::new(mock));
        assert_eq!(Some(op), data.put(&entity).unwrap());
    }
    #[test]
    fn test_data_del_existent() {
        let id = EntityID::ResourceID("an id".to_owned());
        let entity = Entity::Resource(id.clone(), logic::new_resource_function("name"));
        let get_return = Some(entity.clone());
        let op = Op::Delete {
            id: id.clone(),
            old: entity.clone(),
        };
        let mut mock = MockDatastore::new();
        mock.expect_get()
            .with(eq(id.clone()))
            .times(1)
            .returning(move |_| Ok(get_return.clone()));
        mock.expect_set()
            .with(eq(op.clone()))
            .times(1)
            .returning(|op| Ok(op));
        let data = new(Arc::new(mock));
        assert_eq!(Some(op), data.del(&id).unwrap());
    }
}
