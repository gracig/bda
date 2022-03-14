use bdaproto::Resource;
use std::fmt::Debug;

#[cfg(test)]
use mockall::{automock, predicate::*};

use self::query::Query;

pub mod datastore;
pub mod query;

type Entities = Box<dyn Iterator<Item = (EntityID, Entity)>>;

#[cfg_attr(test, automock)]
pub trait Datastore {
    fn get<'a>(&self, id: &'a EntityID) -> Result<Option<Entity>, String>;
    fn set(&self, action: Op) -> Result<Op, String>;
    fn search<'a>(&self, query: &'a Query) -> Result<Entities, String>;
}

pub fn new<'d, D: Datastore>(datastore: &'d D) -> Data<'d> {
    Data::new(datastore)
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityKind {
    Resource,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum EntityID {
    ResourceID(String),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Entity {
    Resource(Resource),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Create {
        id: EntityID,
        new: Entity,
    },
    Update {
        id: EntityID,
        new: Entity,
        old: Entity,
    },
    Delete {
        id: EntityID,
        old: Entity,
    },
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
    pub fn put<'a>(&self, id: &EntityID, new: &Entity) -> Result<Option<Op>, String> {
        match self.datastore.get(&id)? {
            None => Ok(Some(self.datastore.set(Op::Create {
                id: id.clone(),
                new: new.clone(),
            })?)),
            Some(old) => {
                if *new == old {
                    Ok(None)
                } else {
                    Ok(Some(self.datastore.set(Op::Update {
                        id: id.clone(),
                        new: new.clone(),
                        old,
                    })?))
                }
            }
        }
    }
    pub fn search<'a>(&self, query: &'a Query) -> Result<Entities, String> {
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
        let entity = Entity::Resource(factory::new_resource_function("name"));
        let items = vec![
            (EntityID::ResourceID("a".to_owned()), entity.clone()),
            (EntityID::ResourceID("b".to_owned()), entity.clone()),
        ];
        let items2 = vec![
            (EntityID::ResourceID("a".to_owned()), entity.clone()),
            (EntityID::ResourceID("c".to_owned()), entity.clone()),
        ];
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
        let entity = Entity::Resource(factory::new_resource_function("name"));
        let op = Op::Create {
            id: id.clone(),
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
        assert_eq!(Some(op), data.put(&id, &entity).unwrap());
    }
    #[test]
    fn test_data_put_same() {
        let id = EntityID::ResourceID("an id".to_owned());
        let entity = Entity::Resource(factory::new_resource_function("name"));
        let get_entity = entity.clone();
        let get_return = Some(get_entity);
        let mut mock = MockDatastore::new();
        mock.expect_get()
            .with(eq(id.clone()))
            .times(1)
            .returning(move |_| Ok(get_return.clone()));
        mock.expect_set().times(0);
        let data = new(&mock);
        assert_eq!(None, data.put(&id, &entity).unwrap());
    }
    #[test]
    fn test_data_put_change() {
        let id = EntityID::ResourceID("an id".to_owned());
        let entity = Entity::Resource(factory::new_resource_function("name"));
        let mut get_entity = entity.clone();
        let Entity::Resource(ref mut r) = get_entity;
        r.description = "last description".to_owned();
        let get_return = Some(get_entity.clone());
        let op = Op::Update {
            id: id.clone(),
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
        assert_eq!(Some(op), data.put(&id, &entity).unwrap());
    }
    #[test]
    fn test_data_del_existent() {
        let id = EntityID::ResourceID("an id".to_owned());
        let entity = Entity::Resource(factory::new_resource_function("name"));
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

pub mod traits {
    use bdaproto::Resource;
    pub trait Validator {
        fn validate(&self, r: &mut Resource) -> Result<(), String>;
    }
    pub trait Indexer {
        fn index<'a>(&self, action: &'a Action<'a>) -> Result<(), String>;
    }
    pub trait Datastore {
        fn get<'a>(&self, id: &'a str) -> Result<Option<Resource>, String>;
        fn update<'a>(&self, action: &'a Action<'a>) -> Result<(), String>;
    }

    pub trait Auditor {
        fn audit<'a>(&self, action: &'a Action<'a>) -> Result<(), String>;
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Action<'a> {
        Create {
            id: &'a str,
            new: &'a Resource,
        },
        Update {
            id: &'a str,
            new: &'a Resource,
            old: Resource,
        },
        Delete {
            id: &'a str,
            old: Resource,
        },
    }
    #[derive(Debug, Clone)]
    pub enum Error<'a> {
        ErrOnValidate(String),
        ErrOnDBGet(String),
        ErrOnDBPut(String, Action<'a>),
        ErrOnIndex(String, Action<'a>),
        ErrOnAudit(String, Action<'a>),
    }

    pub fn del<'a>(opt: &'a Opt, id: &'a str) -> Result<Option<Action<'a>>, Error<'a>> {
        match opt.d.get(id) {
            Ok(None) => Ok(None),
            Ok(Some(old)) => perform_action(opt, Action::Delete { id, old }),
            Err(s) => Err(Error::ErrOnDBGet(s)),
        }
    }

    pub struct Opt<'a> {
        v: &'a dyn Validator,
        i: &'a dyn Indexer,
        d: &'a dyn Datastore,
        a: &'a dyn Auditor,
    }

    pub fn put<'a>(
        opt: &'a Opt,
        id: &'a str,
        new: &'a mut Resource,
    ) -> Result<Option<Action<'a>>, Error<'a>> {
        match opt.d.get(id) {
            Ok(None) => {
                if let Err(s) = opt.v.validate(new) {
                    return Err(Error::ErrOnValidate(s));
                }
                perform_action(opt, Action::Create { id, new })
            }
            Ok(Some(old)) => {
                if new == &old {
                    Ok(None)
                } else {
                    perform_action(opt, Action::Update { id, new, old })
                }
            }
            Err(s) => Err(Error::ErrOnDBGet(s)),
        }
    }

    fn perform_action<'a>(opt: &'a Opt, a: Action<'a>) -> Result<Option<Action<'a>>, Error<'a>> {
        if let Err(s) = opt.d.update(&a) {
            Err(Error::ErrOnDBPut(s, a))
        } else if let Err(s) = opt.i.index(&a) {
            Err(Error::ErrOnIndex(s, a))
        } else if let Err(s) = opt.a.audit(&a) {
            Err(Error::ErrOnAudit(s, a))
        } else {
            Ok(Some(a))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::model::factory;
        use core::panic;

        #[test]
        fn test_put_new_resource_without_errors() -> () {
            let id = "some/id";
            let mut new = factory::new_resource_function("a function");
            let new_ref = &new.clone();
            let action = Some(Action::Create { id, new: new_ref });
            let v = validator::Mock::new();
            let i = indexer::Mock::new();
            let d = datastore::Mock::new();
            let a = auditor::Mock::new();
            let opt: Opt<'_> = Opt {
                v: v.validate_resource_arg(new_ref)
                    .validate_calls(1)
                    .validate_return(Ok(())),
                i: i.index_action_arg(&action)
                    .index_calls(1)
                    .index_return(Ok(())),
                d: d.get_id_arg(id)
                    .get_calls(1)
                    .get_return(Ok(None))
                    .update_action_arg(&action)
                    .update_calls(1)
                    .update_return(Ok(())),
                a: a.audit_action_arg(&action)
                    .audit_calls(1)
                    .audit_return(Ok(())),
            };
            match put(&opt, id.clone(), &mut new) {
                Ok(a) => match a {
                    Some(b) => assert_eq!(b, action.clone().unwrap()),
                    None => {}
                },
                Err(e) => {
                    panic!("error not expected {:?}", e)
                }
            };
        }

        #[test]
        fn test_put_new_resource_with_validation_error() -> () {
            let id = "some/id";
            let mut new = factory::new_resource_function("a function");
            let new_ref = &new.clone();
            let action = Some(Action::Create { id, new: new_ref });
            let v = validator::Mock::new();
            let i = indexer::Mock::new();
            let d = datastore::Mock::new();
            let a = auditor::Mock::new();
            let err_msg = "a validation error has occurred";
            let opt: Opt<'_> = Opt {
                v: v.validate_resource_arg(new_ref)
                    .validate_calls(1)
                    .validate_return(Err(err_msg.to_owned())),
                i: i.index_action_arg(&action)
                    .index_calls(0)
                    .index_return(Ok(())),
                d: d.get_id_arg(id)
                    .get_calls(1)
                    .get_return(Ok(None))
                    .update_action_arg(&action)
                    .update_calls(0)
                    .update_return(Ok(())),
                a: a.audit_action_arg(&action)
                    .audit_calls(0)
                    .audit_return(Ok(())),
            };
            match put(&opt, id.clone(), &mut new) {
                Err(Error::ErrOnValidate(e)) => {
                    assert_eq!(e, err_msg);
                    ()
                }
                other => panic!("Expecting ErrOnValidate error but got {:?} instead", other),
            };
        }

        #[test]
        fn test_put_new_resource_with_dbget_error() -> () {
            let id = "some/id";
            let mut new = factory::new_resource_function("a function");
            let new_ref = &new.clone();
            let action = Some(Action::Create { id, new: new_ref });
            let v = validator::Mock::new();
            let i = indexer::Mock::new();
            let d = datastore::Mock::new();
            let a = auditor::Mock::new();
            let err_msg = "a db get error has occurred";
            let opt: Opt<'_> = Opt {
                v: v.validate_resource_arg(new_ref)
                    .validate_calls(0)
                    .validate_return(Ok(())),
                i: i.index_action_arg(&action)
                    .index_calls(0)
                    .index_return(Ok(())),
                d: d.get_id_arg(id)
                    .get_calls(1)
                    .get_return(Err(err_msg.to_owned()))
                    .update_action_arg(&action)
                    .update_calls(0)
                    .update_return(Ok(())),
                a: a.audit_action_arg(&action)
                    .audit_calls(0)
                    .audit_return(Ok(())),
            };
            match put(&opt, id.clone(), &mut new) {
                Err(Error::ErrOnDBGet(e)) => {
                    assert_eq!(e, err_msg);
                    ()
                }
                other => panic!("Expecting ErrOnDBGet but got {:?} instead", other),
            };
        }

        #[test]
        fn test_put_new_resource_with_index_error() -> () {
            let id = "some/id";
            let mut new = factory::new_resource_function("a function");
            let new_ref = &new.clone();
            let action = Some(Action::Create { id, new: new_ref });
            let v = validator::Mock::new();
            let i = indexer::Mock::new();
            let d = datastore::Mock::new();
            let a = auditor::Mock::new();
            let err_msg = "an index error has occurred";
            let opt: Opt<'_> = Opt {
                v: v.validate_resource_arg(new_ref)
                    .validate_calls(1)
                    .validate_return(Ok(())),
                i: i.index_action_arg(&action)
                    .index_calls(1)
                    .index_return(Err(err_msg.to_owned())),
                d: d.get_id_arg(id)
                    .get_calls(1)
                    .get_return(Ok(None))
                    .update_action_arg(&action)
                    .update_calls(1)
                    .update_return(Ok(())),
                a: a.audit_action_arg(&action)
                    .audit_calls(0)
                    .audit_return(Ok(())),
            };
            match put(&opt, id.clone(), &mut new) {
                Err(Error::ErrOnIndex(e, a)) => {
                    assert_eq!(e, err_msg);
                    assert_eq!(a, action.clone().unwrap());
                    ()
                }
                other => panic!("Expecting ErrOnIndex but got {:?} instead", other),
            };
        }
        #[test]
        fn test_put_new_resource_with_dbput_error() -> () {
            let id = "some/id";
            let mut new = factory::new_resource_function("a function");
            let new_ref = &new.clone();
            let action = Some(Action::Create { id, new: new_ref });
            let v = validator::Mock::new();
            let i = indexer::Mock::new();
            let d = datastore::Mock::new();
            let a = auditor::Mock::new();
            let err_msg = "a db put error has occurred";
            let opt: Opt<'_> = Opt {
                v: v.validate_resource_arg(new_ref)
                    .validate_calls(1)
                    .validate_return(Ok(())),
                i: i.index_action_arg(&action)
                    .index_calls(0)
                    .index_return(Ok(())),
                d: d.get_id_arg(id)
                    .get_calls(1)
                    .get_return(Ok(None))
                    .update_action_arg(&action)
                    .update_calls(1)
                    .update_return(Err(err_msg.to_owned())),
                a: a.audit_action_arg(&action)
                    .audit_calls(0)
                    .audit_return(Ok(())),
            };
            match put(&opt, id.clone(), &mut new) {
                Err(Error::ErrOnDBPut(e, a)) => {
                    assert_eq!(e, err_msg);
                    assert_eq!(a, action.clone().unwrap());
                    ()
                }
                other => panic!("Expecting ErrOnDBPut but got {:?} instead", other),
            };
        }

        #[test]
        fn test_put_new_resource_with_audit_error() -> () {
            let id = "some/id";
            let mut new = factory::new_resource_function("a function");
            let new_ref = &new.clone();
            let action = Some(Action::Create { id, new: new_ref });
            let v = validator::Mock::new();
            let i = indexer::Mock::new();
            let d = datastore::Mock::new();
            let a = auditor::Mock::new();
            let err_msg = "an audit error has occurred";
            let opt: Opt<'_> = Opt {
                v: v.validate_resource_arg(new_ref)
                    .validate_calls(1)
                    .validate_return(Ok(())),
                i: i.index_action_arg(&action)
                    .index_calls(1)
                    .index_return(Ok(())),
                d: d.get_id_arg(id)
                    .get_calls(1)
                    .get_return(Ok(None))
                    .update_action_arg(&action)
                    .update_calls(1)
                    .update_return(Ok(())),
                a: a.audit_action_arg(&action)
                    .audit_calls(1)
                    .audit_return(Err(err_msg.to_owned())),
            };
            match put(&opt, id.clone(), &mut new) {
                Err(Error::ErrOnAudit(e, a)) => {
                    assert_eq!(e, err_msg);
                    assert_eq!(a, action.clone().unwrap());
                    ()
                }
                other => panic!("Expecting ErrOnAudit but got {:?} instead", other),
            };
        }
        #[test]
        fn test_update_eq_resource_without_errors() -> () {
            let id = "some/id";
            let mut new = factory::new_resource_function("a function");
            let new_ref = &new.clone();
            let old = new.clone();
            let action = None;
            let v = validator::Mock::new();
            let i = indexer::Mock::new();
            let d = datastore::Mock::new();
            let a = auditor::Mock::new();
            let opt: Opt<'_> = Opt {
                v: v.validate_resource_arg(new_ref)
                    .validate_calls(0)
                    .validate_return(Ok(())),
                i: i.index_action_arg(&action)
                    .index_calls(0)
                    .index_return(Ok(())),
                d: d.get_id_arg(id)
                    .get_calls(1)
                    .get_return(Ok(Some(old))) //get simulating an already existent eq resource
                    .update_action_arg(&action)
                    .update_calls(0)
                    .update_return(Ok(())),
                a: a.audit_action_arg(&action)
                    .audit_calls(0)
                    .audit_return(Ok(())),
            };
            match put(&opt, id.clone(), &mut new) {
                Ok(a) => match a {
                    Some(b) => assert_eq!(b, action.clone().unwrap()),
                    None => {}
                },
                Err(e) => {
                    panic!("error not expected {:?}", e)
                }
            };
        }
        #[test]
        fn test_update_ne_resource_without_errors() -> () {
            let id = "some/id";
            let mut new = factory::new_resource_function("a function");
            let new_ref = &new.clone();
            let mut old = new.clone();
            old.description = "I am a different beast".to_owned();
            let action = Some(Action::Update {
                id,
                new: new_ref,
                old: old.clone(),
            });
            let v = validator::Mock::new();
            let i = indexer::Mock::new();
            let d = datastore::Mock::new();
            let a = auditor::Mock::new();
            let opt: Opt<'_> = Opt {
                v: v.validate_resource_arg(new_ref)
                    .validate_calls(0) //do not need to validate
                    .validate_return(Ok(())),
                i: i.index_action_arg(&action)
                    .index_calls(1)
                    .index_return(Ok(())),
                d: d.get_id_arg(id)
                    .get_calls(1)
                    .get_return(Ok(Some(old))) //get simulating an already existent eq resource
                    .update_action_arg(&action)
                    .update_calls(1)
                    .update_return(Ok(())),
                a: a.audit_action_arg(&action)
                    .audit_calls(1)
                    .audit_return(Ok(())),
            };
            match put(&opt, id.clone(), &mut new) {
                Ok(a) => match a {
                    Some(b) => assert_eq!(b, action.clone().unwrap()),
                    None => {}
                },
                Err(e) => {
                    panic!("error not expected {:?}", e)
                }
            };
        }

        pub mod indexer {
            use self::panic;
            use super::*;
            use std::cell::{Cell, RefCell};
            pub struct Mock<'a> {
                index_action: RefCell<Option<&'a Option<Action<'a>>>>,
                index_calls: Cell<usize>,
                index_return: RefCell<Option<Result<(), String>>>,
                index_calls_counter: Cell<usize>,
            }

            impl<'a> Mock<'a> {
                pub fn index_action_arg(&self, action: &'a Option<Action<'a>>) -> &Self {
                    self.index_action.replace(Some(action));
                    self
                }
                pub fn index_calls(&self, calls: usize) -> &Self {
                    self.index_calls.set(calls);
                    self
                }
                pub fn index_return(&self, ret: Result<(), String>) -> &Self {
                    self.index_return.replace(Some(ret));
                    self
                }
                pub fn new() -> Mock<'a> {
                    return Mock {
                        index_action: RefCell::new(None),
                        index_calls: Cell::new(0),
                        index_calls_counter: Cell::new(0),
                        index_return: RefCell::new(None),
                    };
                }
                pub fn check_calls(&self) {
                    assert_eq!(self.index_calls_counter.get(), self.index_calls.get())
                }
            }
            impl Drop for Mock<'_> {
                fn drop<'a>(&'a mut self) {
                    self.check_calls();
                }
            }
            impl Indexer for Mock<'_> {
                fn index<'a>(&self, action: &'a Action<'a>) -> Result<(), String> {
                    self.index_calls_counter
                        .set(self.index_calls_counter.get() + 1);
                    match *self.index_action.borrow() {
                        Some(Some(my_action)) => {
                            assert_eq!(action, my_action);
                            self.index_return
                                .borrow()
                                .clone()
                                .ok_or_else(|| "index_return is undefined")?
                        }
                        Some(None) => panic!("this function should have never be called"),
                        None => panic!("index_action was not defined on mock"),
                    }
                }
            }
        }

        pub mod auditor {
            use self::panic;
            use super::*;
            use std::cell::{Cell, RefCell};
            pub struct Mock<'a> {
                audit_action: RefCell<Option<&'a Option<Action<'a>>>>,
                audit_calls: Cell<usize>,
                audit_calls_counter: Cell<usize>,
                audit_return: RefCell<Option<Result<(), String>>>,
            }

            impl<'a> Mock<'a> {
                pub fn audit_action_arg(&self, action: &'a Option<Action<'a>>) -> &Self {
                    self.audit_action.replace(Some(action));
                    self
                }
                pub fn audit_calls(&self, calls: usize) -> &Self {
                    self.audit_calls.set(calls);
                    self
                }
                pub fn audit_return(&self, ret: Result<(), String>) -> &Self {
                    self.audit_return.replace(Some(ret));
                    self
                }
                pub fn new() -> Mock<'a> {
                    return Mock {
                        audit_action: RefCell::new(None),
                        audit_calls: Cell::new(0),
                        audit_calls_counter: Cell::new(0),
                        audit_return: RefCell::new(None),
                    };
                }
                pub fn check_calls(&self) {
                    assert_eq!(self.audit_calls_counter.get(), self.audit_calls.get())
                }
            }
            impl Drop for Mock<'_> {
                fn drop<'a>(&'a mut self) {
                    self.check_calls();
                }
            }
            impl Auditor for Mock<'_> {
                fn audit<'a>(&self, action: &'a Action<'a>) -> Result<(), String> {
                    self.audit_calls_counter
                        .set(self.audit_calls_counter.get() + 1);
                    match *self.audit_action.borrow() {
                        Some(Some(my_action)) => {
                            assert_eq!(action, my_action);
                            self.audit_return
                                .borrow()
                                .clone()
                                .ok_or_else(|| "audit_return is undefined")?
                        }
                        Some(None) => panic!("this function should have never be called"),
                        None => panic!("audit_action was not defined on mock"),
                    }
                }
            }
        }

        pub mod validator {
            use self::panic;
            use super::*;
            use std::cell::{Cell, RefCell};
            pub struct Mock<'a> {
                validate_resource: RefCell<Option<&'a Resource>>,
                validate_calls: Cell<usize>,
                validate_calls_counter: Cell<usize>,
                validate_return: RefCell<Option<Result<(), String>>>,
            }

            impl<'a> Mock<'a> {
                pub fn validate_resource_arg(&self, resource: &'a Resource) -> &Self {
                    self.validate_resource.replace(Some(resource));
                    self
                }
                pub fn validate_calls(&self, calls: usize) -> &Self {
                    self.validate_calls.set(calls);
                    self
                }
                pub fn validate_return(&self, ret: Result<(), String>) -> &Self {
                    self.validate_return.replace(Some(ret));
                    self
                }
                pub fn new() -> Mock<'a> {
                    return Mock {
                        validate_resource: RefCell::new(None),
                        validate_calls: Cell::new(0),
                        validate_calls_counter: Cell::new(0),
                        validate_return: RefCell::new(None),
                    };
                }
                pub fn check_calls(&self) {
                    assert_eq!(self.validate_calls_counter.get(), self.validate_calls.get())
                }
            }
            impl Drop for Mock<'_> {
                fn drop<'a>(&'a mut self) {
                    self.check_calls();
                }
            }
            impl Validator for Mock<'_> {
                fn validate(&self, r: &mut Resource) -> Result<(), String> {
                    self.validate_calls_counter
                        .set(self.validate_calls_counter.get() + 1);
                    match *self.validate_resource.borrow() {
                        Some(my_r) => {
                            assert_eq!(r, my_r);
                            self.validate_return
                                .borrow()
                                .clone()
                                .ok_or_else(|| "validate_return is undefined")?
                        }
                        None => panic!("validate_resource is undefined"),
                    }
                }
            }
        }
        pub mod datastore {
            use self::panic;
            use super::*;
            use std::cell::{Cell, RefCell};
            pub struct Mock<'a> {
                get_id: RefCell<Option<&'a str>>,
                get_calls: Cell<usize>,
                get_return: RefCell<Option<Result<Option<Resource>, String>>>,
                get_calls_counter: Cell<usize>,
                update_action: RefCell<Option<&'a Option<Action<'a>>>>,
                update_calls: Cell<usize>,
                update_return: RefCell<Option<Result<(), String>>>,
                update_calls_counter: Cell<usize>,
            }

            impl<'a> Mock<'a> {
                pub fn new() -> Mock<'a> {
                    return Mock {
                        get_id: RefCell::new(None),
                        get_calls: Cell::new(0),
                        get_return: RefCell::new(None),
                        get_calls_counter: Cell::new(0),
                        update_action: RefCell::new(None),
                        update_calls: Cell::new(0),
                        update_return: RefCell::new(None),
                        update_calls_counter: Cell::new(0),
                    };
                }
                pub fn get_id_arg(&self, id: &'a str) -> &Self {
                    self.get_id.replace(Some(id));
                    self
                }
                pub fn get_calls(&self, calls: usize) -> &Self {
                    self.get_calls.set(calls);
                    self
                }
                pub fn get_return(&self, ret: Result<Option<Resource>, String>) -> &Self {
                    self.get_return.replace(Some(ret));
                    self
                }
                pub fn update_action_arg(&self, action: &'a Option<Action<'a>>) -> &Self {
                    self.update_action.replace(Some(action));
                    self
                }
                pub fn update_calls(&self, calls: usize) -> &Self {
                    self.update_calls.set(calls);
                    self
                }
                pub fn update_return(&self, ret: Result<(), String>) -> &Self {
                    self.update_return.replace(Some(ret));
                    self
                }
                pub fn check_calls(&self) {
                    assert_eq!(self.get_calls_counter.get(), self.get_calls.get());
                    assert_eq!(self.update_calls_counter.get(), self.update_calls.get());
                }
            }
            impl Drop for Mock<'_> {
                fn drop<'a>(&'a mut self) {
                    self.check_calls();
                }
            }
            impl Datastore for Mock<'_> {
                fn get<'a>(&self, id: &'a str) -> Result<Option<Resource>, String> {
                    self.get_calls_counter.set(self.get_calls_counter.get() + 1);
                    match *self.get_id.borrow() {
                        Some(my_id) => {
                            assert_eq!(id, my_id);
                            self.get_return
                                .borrow()
                                .clone()
                                .ok_or_else(|| "get_return is undefined")?
                        }
                        None => panic!("get_id was not defined on mock"),
                    }
                }
                fn update<'a>(&self, action: &'a Action<'a>) -> Result<(), String> {
                    self.update_calls_counter
                        .set(self.update_calls_counter.get() + 1);
                    match *self.update_action.borrow() {
                        Some(Some(my_action)) => {
                            assert_eq!(action, my_action);
                            self.update_return
                                .borrow()
                                .clone()
                                .ok_or_else(|| "update_return is undefined")?
                        }
                        Some(None) => panic!("this function should have never be called"),
                        None => panic!("update_id is undefined"),
                    }
                }
            }
        }
    }
}
