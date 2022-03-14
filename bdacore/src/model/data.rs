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

pub struct Opt<'a> {
    v: &'a dyn Validator,
    i: &'a dyn Indexer,
    d: &'a dyn Datastore,
    a: &'a dyn Auditor,
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

pub fn put<'a>(
    opt: &'a Opt,
    id: &'a str,
    new: &'a mut Resource,
) -> Result<Option<Action<'a>>, Error<'a>> {
    if let Err(s) = opt.v.validate(new) {
        return Err(Error::ErrOnValidate(s));
    }
    match opt.d.get(id) {
        Ok(None) => perform_action(opt, Action::Create { id, new }),
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
        let action = Action::Create { id, new: new_ref };
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
                Some(b) => assert_eq!(&b, &action.clone()),
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
        let action = Action::Create { id, new: new_ref };
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
                .get_calls(0)
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
        let action = Action::Create { id, new: new_ref };
        let v = validator::Mock::new();
        let i = indexer::Mock::new();
        let d = datastore::Mock::new();
        let a = auditor::Mock::new();
        let err_msg = "a db get error has occurred";
        let opt: Opt<'_> = Opt {
            v: v.validate_resource_arg(new_ref)
                .validate_calls(1)
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
        let action = Action::Create { id, new: new_ref };
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
                assert_eq!(a, action);
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
        let action = Action::Create { id, new: new_ref };
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
                assert_eq!(a, action);
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
        let action = Action::Create { id, new: new_ref };
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
                assert_eq!(a, action);
                ()
            }
            other => panic!("Expecting ErrOnAudit but got {:?} instead", other),
        };
    }

    pub mod indexer {
        use self::panic;
        use super::*;
        use std::cell::{Cell, RefCell};
        pub struct Mock<'a> {
            index_action: RefCell<Option<&'a Action<'a>>>,
            index_calls: Cell<usize>,
            index_return: RefCell<Option<Result<(), String>>>,
            index_calls_counter: Cell<usize>,
        }

        impl<'a> Mock<'a> {
            pub fn index_action_arg(&self, action: &'a Action<'a>) -> &Self {
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
                    Some(my_action) => {
                        assert_eq!(action, my_action);
                        self.index_return
                            .borrow()
                            .clone()
                            .ok_or_else(|| "index_return is undefined")?
                    }
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
            audit_action: RefCell<Option<&'a Action<'a>>>,
            audit_calls: Cell<usize>,
            audit_calls_counter: Cell<usize>,
            audit_return: RefCell<Option<Result<(), String>>>,
        }

        impl<'a> Mock<'a> {
            pub fn audit_action_arg(&self, action: &'a Action<'a>) -> &Self {
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
                    Some(my_action) => {
                        assert_eq!(action, my_action);
                        self.audit_return
                            .borrow()
                            .clone()
                            .ok_or_else(|| "audit_return is undefined")?
                    }
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
            update_action: RefCell<Option<&'a Action<'a>>>,
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
            pub fn update_action_arg(&self, action: &'a Action<'a>) -> &Self {
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
                    Some(my_action) => {
                        assert_eq!(action, my_action);
                        self.update_return
                            .borrow()
                            .clone()
                            .ok_or_else(|| "update_return is undefined")?
                    }
                    None => panic!("update_id is undefined"),
                }
            }
        }
    }
}
