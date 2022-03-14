use bdaproto::Resource;

pub trait Validator {
    fn validate(&self, r: &mut Resource) -> Result<(), String>;
}
pub trait Indexer {
    fn index(&self, id: &str, action: &Option<Action>) -> Result<(), String>;
}
pub trait Datastore {
    fn put(&self, id: &str, r: &Resource) -> Result<(), String>;
    fn get(&self, id: &str) -> Result<Option<Resource>, String>;
}
pub trait Audit {
    fn audit(&self, id: &str, action: &Option<Action>) -> Result<(), String>;
}
pub struct Options {
    v: Box<dyn Validator>,
    i: Box<dyn Indexer>,
    d: Box<dyn Datastore>,
    a: Box<dyn Audit>,
}

#[derive(Debug, Clone)]
pub enum Action {
    Create { new: Resource },
    Update { new: Resource, old: Resource },
}
#[derive(Debug, Clone)]
pub enum Error {
    ErrOnValidate(String),
    ErrOnDBGet(String),
    ErrOnDBPut(String, Option<Action>),
    ErrOnIndex(String, Option<Action>),
    ErrOnAudit(String, Option<Action>),
}

pub fn put_resource(opt: Options, id: &str, r: &mut Resource) -> Result<Option<Action>, Error> {
    //validates resource. may mutate to insert defaults or to fulfill any other possible rule
    if let Err(s) = opt.v.validate(r) {
        return Err(Error::ErrOnValidate(s));
    }
    //find out which action to take, based on querying possible previous version of the resource
    let action;
    match opt.d.get(id) {
        Err(s) => return Err(Error::ErrOnDBGet(s)),
        Ok(None) => {
            action = Some(Action::Create { new: r.clone() });
        }
        Ok(Some(old)) => {
            if r != &old {
                action = Some(Action::Update {
                    new: r.clone(),
                    old,
                });
            } else {
                action = None;
            }
        }
    }
    //acting when action is not None
    if let Some(_) = action {
        if let Err(s) = opt.d.put(id, r) {
            return Err(Error::ErrOnDBPut(s, action));
        }
        if let Err(s) = opt.i.index(id, &action) {
            return Err(Error::ErrOnIndex(s, action));
        }
        if let Err(s) = opt.a.audit(id, &action) {
            return Err(Error::ErrOnAudit(s, action));
        }
    }
    Ok(action)
}
