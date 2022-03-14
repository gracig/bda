pub mod datastore;
pub mod query;

use crate::{data::query::Query, logic};
use bdaindex::bql::Value;
use bdaproto::Resource;
use std::{error::Error, fmt::Debug, sync::Arc};

#[cfg(test)]
use mockall::{automock, predicate::*};

type EntityIDIterator = Box<dyn Iterator<Item = Result<EntityID, Box<dyn Error>>>>;
type ValueIterator = Box<dyn Iterator<Item = Result<Value, Box<dyn Error>>>>;

#[cfg_attr(test, automock)]
pub trait Datastore {
    fn get<'a>(&self, id: &'a EntityID) -> Result<Option<Entity>, Box<dyn Error>>;
    fn set(&self, action: Op) -> Result<Op, Box<dyn Error>>;
    fn search<'a>(&self, query: &'a Query) -> Result<EntityIDIterator, Box<dyn Error>>;
    fn values<'a>(
        &self,
        kind: &'a EntityKind,
        field: &'a str,
    ) -> Result<ValueIterator, Box<dyn Error>>;
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

    fn get<'a>(&self, id: &'a EntityID) -> Result<Option<Entity>, Box<dyn Error>> {
        self.datastore.get(id)
    }

    pub fn del<'a>(&self, id: &'a EntityID) -> Result<Option<Op>, Box<dyn Error>> {
        match self.datastore.get(id)? {
            None => Ok(None),
            Some(old) => Ok(Some(self.datastore.set(Op::Delete {
                id: id.clone(),
                old,
            })?)),
        }
    }
    fn put<'a>(&self, new: &Entity) -> Result<Option<Op>, Box<dyn Error>> {
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

    pub fn get_resource<'a>(&self, id: &'a EntityID) -> Result<Option<Resource>, Box<dyn Error>> {
        self.get(id).map(|oe| {
            oe.map(|entity| match entity {
                Entity::Resource(_, r) => r,
            })
        })
    }

    pub fn put_resource<'a>(&self, r: &Resource) -> Result<Option<Op>, Box<dyn Error>> {
        let mut validated = r.to_owned();
        logic::defaults(&mut validated);
        self.put(&Entity::Resource(
            logic::resource_id(&validated)?,
            validated,
        ))
    }

    pub fn search<'a>(&self, query: &'a Query) -> Result<EntityIDIterator, Box<dyn Error>> {
        self.datastore.search(query)
    }

    pub fn ids<'a>(&self, query: &'a Query) -> Result<Vec<EntityID>, Box<dyn Error>> {
        self.search(query).and_then(|mut iter| {
            iter.try_fold(Vec::new(), |mut acc, item| {
                item.and_then(|id| {
                    acc.push(id);
                    Ok(acc)
                })
            })
        })
    }

    pub fn resources<'a>(&self, query: &'a Query) -> Result<Vec<Resource>, Box<dyn Error>> {
        match query.kind {
            EntityKind::Resource => self.search(query).and_then(|mut iter| {
                iter.try_fold(Vec::new(), |mut acc, item| {
                    item.and_then(|ref id| {
                        self.datastore.get(id).and_then(|r| match r {
                            Some(e) => match e {
                                Entity::Resource(_, r) => {
                                    acc.push(r);
                                    Ok(acc)
                                }
                            },
                            None => Err(format!("resource not found for id: {:?}", id))?,
                        })
                    })
                })
            }),
        }
    }

    pub fn values<'a>(
        &self,
        kind: &'a EntityKind,
        field: &'a str,
    ) -> Result<ValueIterator, Box<dyn Error>> {
        self.datastore.values(kind, field)
    }

    pub fn values_as_string<'a>(
        &self,
        kind: &'a EntityKind,
        field: &'a str,
    ) -> Result<Box<dyn Iterator<Item = Result<String, Box<dyn Error>>>>, Box<dyn Error>> {
        self.datastore.values(kind, field).and_then(|iter| {
            Ok(Box::new(iter.filter_map(|rv| match rv {
                Ok(v) => match v {
                    Value::Rational(vv) => Some(Ok(vv.to_string())),
                    Value::Text(vv) => Some(Ok(vv)),
                    Value::Boolean(vv) => Some(Ok(vv.to_string())),
                    Value::Integral(vv) => Some(Ok(vv.to_string())),
                    Value::Bottom => None,
                    Value::Top => None,
                },
                Err(e) => Some(Err(e)),
            }))
                as Box<dyn Iterator<Item = Result<String, Box<dyn Error>>>>)
        })
    }

    pub fn values_as_f64<'a>(
        &self,
        kind: &'a EntityKind,
        field: &'a str,
    ) -> Result<Box<dyn Iterator<Item = Result<f64, Box<dyn Error>>>>, Box<dyn Error>> {
        self.datastore.values(kind, field).and_then(|iter| {
            Ok(Box::new(iter.filter_map(|rv| match rv {
                Ok(v) => match v {
                    Value::Rational(vv) => Some(Ok(vv.value)),
                    Value::Integral(vv) => Some(Ok(vv as f64)),
                    _ => None,
                },
                Err(e) => Some(Err(e)),
            }))
                as Box<dyn Iterator<Item = Result<f64, Box<dyn Error>>>>)
        })
    }

    pub fn values_as_bool<'a>(
        &self,
        kind: &'a EntityKind,
        field: &'a str,
    ) -> Result<Box<dyn Iterator<Item = Result<bool, Box<dyn Error>>>>, Box<dyn Error>> {
        self.datastore.values(kind, field).and_then(|iter| {
            Ok(Box::new(iter.filter_map(|rv| match rv {
                Ok(v) => match v {
                    Value::Boolean(vv) => Some(Ok(vv)),
                    _ => None,
                },
                Err(e) => Some(Err(e)),
            }))
                as Box<dyn Iterator<Item = Result<bool, Box<dyn Error>>>>)
        })
    }
}

#[cfg(test)]
mod test_super {
    use crate::logic;

    use super::*;
    use bdaindex::bql::Ast;

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
        let mut set = Box::new(items.clone().into_iter().map(|x| Ok(x))) as EntityIDIterator;
        let search_set = Box::new(items2.clone().into_iter().map(|x| Ok(x)));
        mock.expect_search()
            .with(eq(q.clone()))
            .times(1)
            .returning(move |_| Ok(Box::new(search_set.clone())));
        let data = new(Arc::new(mock));
        let mut answer = data.search(&q).unwrap();
        assert_eq!(
            set.next().unwrap().unwrap(),
            answer.next().unwrap().unwrap()
        );
        assert_ne!(
            set.next().unwrap().unwrap(),
            answer.next().unwrap().unwrap()
        );
        assert_eq!(
            set.next().unwrap().unwrap(),
            answer.next().unwrap().unwrap()
        );
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
