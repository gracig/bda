// Modules
pub mod datastore;
pub mod query;

// Imports
use self::query::Query;
use bdaproto::Resource;
#[cfg(test)]
use mockall::{automock, predicate::*};
use std::fmt::Debug;

#[cfg_attr(test, automock)]
pub trait Datastore {
    fn get<'a>(&self, id: &'a EntityID) -> Result<Option<Entity>, String>;
    fn set(&self, action: Op) -> Result<Op, String>;
    fn search<'a>(&self, query: &'a Query) -> Result<Box<dyn Iterator<Item = EntityID>>, String>;
}

pub fn new<'d, D: Datastore>(datastore: &'d D) -> Data<'d> {
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
    Resource(String, Resource),
}
impl Entity {
    pub fn to_kind(&self) -> EntityKind {
        match self {
            Entity::Resource(_, _) => EntityKind::Resource,
        }
    }
    pub fn id(&self) -> EntityID {
        match self {
            Entity::Resource(id, _) => EntityID::ResourceID(id.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Create { new: Entity },
    Update { new: Entity, old: Entity },
    Delete { id: EntityID, old: Entity },
}
pub struct Data<'a> {
    datastore: &'a dyn Datastore,
}

impl<'d> Data<'d> {
    pub fn new<D: Datastore>(datastore: &'d D) -> Data<'d> {
        Data { datastore }
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
    pub fn put<'a>(&self, new: &Entity) -> Result<Option<Op>, String> {
        match new {
            Entity::Resource(id, _) => {
                match self.datastore.get(&EntityID::ResourceID(id.clone()))? {
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
                }
            }
        }
    }
    pub fn search<'a>(
        &self,
        query: &'a Query,
    ) -> Result<Box<dyn Iterator<Item = EntityID>>, String> {
        self.datastore.search(query)
    }
}

#[cfg(test)]
mod test_super {
    use bdaql::Ast;

    use crate::model::factory;

    use super::*;

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
        let data = new(&mock);
        let mut answer = data.search(&q).unwrap();
        assert_eq!(set.next(), answer.next());
        assert_ne!(set.next(), answer.next());
        assert_eq!(set.next(), answer.next());
    }

    #[test]
    fn test_data_put_new() {
        let id = EntityID::ResourceID("an id".to_owned());
        let entity = Entity::Resource("an id".to_owned(), factory::new_resource_function("name"));
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
        let data = new(&mock);
        assert_eq!(Some(op), data.put(&entity).unwrap());
    }
    #[test]
    fn test_data_put_same() {
        let id = EntityID::ResourceID("an id".to_owned());
        let entity = Entity::Resource("an id".to_owned(), factory::new_resource_function("name"));
        let get_entity = entity.clone();
        let get_return = Some(get_entity);
        let mut mock = MockDatastore::new();
        mock.expect_get()
            .with(eq(id.clone()))
            .times(1)
            .returning(move |_| Ok(get_return.clone()));
        mock.expect_set().times(0);
        let data = new(&mock);
        assert_eq!(None, data.put(&entity).unwrap());
    }
    #[test]
    fn test_data_put_change() {
        let id = EntityID::ResourceID("an id".to_owned());
        let entity = Entity::Resource("an id".to_owned(), factory::new_resource_function("name"));
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
        let data = new(&mock);
        assert_eq!(Some(op), data.put(&entity).unwrap());
    }
    #[test]
    fn test_data_del_existent() {
        let id = EntityID::ResourceID("an id".to_owned());
        let entity = Entity::Resource("an id".to_owned(), factory::new_resource_function("name"));
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
        let data = new(&mock);
        assert_eq!(Some(op), data.del(&id).unwrap());
    }
}
