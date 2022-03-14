use crate::model::data::{Entity, EntityID, EntityKind};
use bdaql::Value;
use ppom::mdb::{Iter, OMap};
use std::ops::RangeBounds;

type FieldName = String;
type EntitySet = OMap<EntityID, bool>;
type ValueSet = OMap<Option<Value>, EntitySet>;
type WorldIndex = OMap<EntityKind, EntitySet>;
type FieldIndex = OMap<(EntityKind, FieldName), EntitySet>;
type ValueIndex = OMap<(EntityKind, FieldName), ValueSet>;

pub struct Index {
    world_index: WorldIndex,
    field_index: FieldIndex,
    value_index: ValueIndex,
}

pub fn new() -> Index {
    Index::new()
}

impl Index {
    pub fn new() -> Index {
        Index {
            world_index: WorldIndex::new(),
            field_index: FieldIndex::new(),
            value_index: ValueIndex::new(),
        }
    }
    pub fn index(&mut self, entity: &Entity) -> Result<(), String> {
        match entity {
            Entity::Resource(_, r) => {
                self.update_indexes(&entity.id(), &to_field_values(to_json_value(r)?))
            }
        }
    }

    pub fn with_kind(&self, kind: &EntityKind) -> Box<dyn Iterator<Item = EntityID>> {
        Box::new(EntityIDIter {
            iter: self
                .world_index
                .get(kind)
                .ok()
                .and_then(|id_set| id_set.0.iter().ok()),
        })
    }
    pub fn with_field(&self, kind: &EntityKind, field: &str) -> Box<dyn Iterator<Item = EntityID>> {
        Box::new(EntityIDIter {
            iter: self
                .field_index
                .get(&(kind.to_owned(), field.to_string()))
                .ok()
                .and_then(|id_set| id_set.0.iter().ok()),
        })
    }
    pub fn with_value_range<R>(
        &self,
        kind: &EntityKind,
        field: &str,
        range: R,
        exclude_value: bool,
        value_to_exclude: &Option<Value>,
    ) -> Box<dyn Iterator<Item = EntityID>>
    where
        R: RangeBounds<Option<Value>>,
    {
        match self
            .value_index
            .get(&(kind.to_owned(), field.to_string()))
            .ok()
            .and_then(|(v_set, _)| v_set.range(range).ok())
            .and_then(|items| {
                let mut stack: Vec<Box<dyn Iterator<Item = EntityID>>> = vec![];
                //Not so functional... :-(
                for (v, id_set) in items {
                    if exclude_value && v == *value_to_exclude {
                    } else {
                        stack.push(Box::new(EntityIDIter {
                            iter: id_set.iter().ok(),
                        }));
                        if stack.len() > 1 {
                            let iter_a = stack.pop().unwrap();
                            let iter_b = stack.pop().unwrap();
                            stack.push(Box::new(EntityIDSetOperationIter::new(
                                SetOperation::Or,
                                iter_a,
                                iter_b,
                            )));
                        }
                    }
                }
                stack.pop()
            }) {
            Some(iter) => iter,
            None => Box::new(EntityIDIter { iter: None }),
        }
    }

    pub fn with_value_lt(
        &self,
        kind: &EntityKind,
        field: &str,
        value: &Option<Value>,
    ) -> Box<dyn Iterator<Item = EntityID>> {
        self.with_value_range(kind, field, ..value.clone(), false, &None)
    }
    pub fn with_value_lte(
        &self,
        kind: &EntityKind,
        field: &str,
        value: &Option<Value>,
    ) -> Box<dyn Iterator<Item = EntityID>> {
        self.with_value_range(kind, field, ..=value.clone(), false, &None)
    }
    pub fn with_value_gt(
        &self,
        kind: &EntityKind,
        field: &str,
        value: &Option<Value>,
    ) -> Box<dyn Iterator<Item = EntityID>> {
        self.with_value_range(kind, field, value.clone().., true, value)
    }
    pub fn with_value_gte(
        &self,
        kind: &EntityKind,
        field: &str,
        value: &Option<Value>,
    ) -> Box<dyn Iterator<Item = EntityID>> {
        self.with_value_range(kind, field, value.clone().., false, &None)
    }

    pub fn with_value_eq(
        &self,
        kind: &EntityKind,
        field: &str,
        value: &Option<Value>,
    ) -> Box<dyn Iterator<Item = EntityID>> {
        Box::new(EntityIDIter {
            iter: self
                .value_index
                .get(&(kind.to_owned(), field.to_string()))
                .ok()
                .and_then(|v_set| v_set.0.get(value).ok())
                .and_then(|id_set| id_set.0.iter().ok()),
        })
    }
    pub fn with_value_ne(
        &self,
        kind: &EntityKind,
        field: &str,
        value: &Option<Value>,
    ) -> Box<dyn Iterator<Item = EntityID>> {
        self.with_diff(
            self.with_field(kind, field),
            Box::new(EntityIDIter {
                iter: self
                    .value_index
                    .get(&(kind.to_owned(), field.to_string()))
                    .ok()
                    .and_then(|v_set| v_set.0.get(value).ok())
                    .and_then(|id_set| id_set.0.iter().ok()),
            }),
        )
    }
    pub fn with_and(
        &self,
        a: Box<dyn Iterator<Item = EntityID>>,
        b: Box<dyn Iterator<Item = EntityID>>,
    ) -> Box<dyn Iterator<Item = EntityID>> {
        Box::new(EntityIDSetOperationIter::new(SetOperation::And, a, b))
    }
    pub fn with_or(
        &self,
        a: Box<dyn Iterator<Item = EntityID>>,
        b: Box<dyn Iterator<Item = EntityID>>,
    ) -> Box<dyn Iterator<Item = EntityID>> {
        Box::new(EntityIDSetOperationIter::new(SetOperation::Or, a, b))
    }
    pub fn with_diff(
        &self,
        a: Box<dyn Iterator<Item = EntityID>>,
        b: Box<dyn Iterator<Item = EntityID>>,
    ) -> Box<dyn Iterator<Item = EntityID>> {
        Box::new(EntityIDSetOperationIter::new(SetOperation::Diff, a, b))
    }

    pub fn has_entity(&self, entity: &Entity) -> bool {
        match self.world_index.get(&entity.to_kind()) {
            Ok((oset, _)) => match oset.get(&entity.id()) {
                Ok((x, _)) => x,
                Err(_) => false,
            },
            Err(_) => false,
        }
    }
    pub fn has_field(&self, entity: &Entity, field: &str) -> bool {
        match self.field_index.get(&(entity.to_kind(), field.to_string())) {
            Ok((id_set, _)) => match id_set.get(&entity.id()) {
                Ok((b, _)) => b,
                Err(_) => false,
            },
            Err(_) => false,
        }
    }
    pub fn has_value(&self, entity: &Entity, field: &str, value: &Option<Value>) -> bool {
        match self.value_index.get(&(entity.to_kind(), field.to_string())) {
            Ok((vset, _)) => match vset.get(value) {
                Ok((id_set, _)) => match id_set.get(&entity.id()) {
                    Ok((b, _)) => b,
                    Err(_) => false,
                },
                Err(_) => false,
            },
            Err(_) => false,
        }
    }

    fn update_indexes(&mut self, id: &EntityID, values: &Vec<FieldValue>) -> Result<(), String> {
        for v in values {
            //println!("{:?}:{:?}", v.accessor_to_string(), v.value);
            self.update_world_index(id)
                .and(self.update_value_index(id, &v.accessor_to_string(), &v.value))
                .and(self.update_field_index(id, &v))?;
        }
        Ok(())
    }

    fn update_world_index(&self, id: &EntityID) -> Result<Option<EntitySet>, String> {
        let id_set = self
            .world_index
            .get(&id.to_kind())
            .unwrap_or((OMap::new(), 0))
            .0;

        Ok(id_set
            .set(id.clone(), true)
            .and(self.world_index.set(id.to_kind(), id_set))
            .map_err(|e| e.to_string())?)
    }

    fn update_value_index(
        &self,
        id: &EntityID,
        field: &str,
        value: &Option<Value>,
    ) -> Result<Option<ValueSet>, String> {
        let v_set = self
            .value_index
            .get(&(id.to_kind(), field.to_string()))
            .unwrap_or((OMap::new(), 0))
            .0;
        let id_set = v_set.get(value).unwrap_or((OMap::new(), 0)).0;
        Ok(id_set
            .set(id.clone(), true)
            .and(v_set.set(value.clone(), id_set))
            .and(
                self.value_index
                    .set((id.to_kind(), field.to_string()), v_set),
            )
            .map_err(|e| e.to_string())?)
    }

    fn update_field_index(&self, id: &EntityID, fv: &FieldValue) -> Result<(), String> {
        let mut cv = fv.clone();
        loop {
            let field = cv.accessor_to_string();
            let id_set = self
                .field_index
                .get(&(id.to_kind(), field.clone()))
                .unwrap_or((OMap::new(), 0))
                .0;
            id_set
                .set(id.clone(), true)
                .and(
                    self.field_index
                        .set((id.to_kind(), field.to_string()), id_set),
                )
                .map_err(|e| e.to_string())?;
            if cv.accessor.is_empty() {
                break;
            }
            cv.accessor.pop();
        }
        Ok(())
    }
}

enum SetOperation {
    And,
    Or,
    Diff,
}
pub struct EntityIDSetOperationIter {
    iter_a: Box<dyn Iterator<Item = EntityID>>,
    iter_b: Box<dyn Iterator<Item = EntityID>>,
    read_a: bool,
    read_b: bool,
    next_a: Option<EntityID>,
    next_b: Option<EntityID>,
    set_op: SetOperation,
}
impl EntityIDSetOperationIter {
    fn new(
        set_op: SetOperation,
        iter_a: Box<dyn Iterator<Item = EntityID>>,
        iter_b: Box<dyn Iterator<Item = EntityID>>,
    ) -> Self {
        EntityIDSetOperationIter {
            iter_a,
            iter_b,
            set_op,
            read_a: true,
            read_b: true,
            next_a: None,
            next_b: None,
        }
    }
}
impl Iterator for EntityIDSetOperationIter {
    type Item = EntityID;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.read_a {
                self.next_a = self.iter_a.next();
                if self.next_a == None {
                    self.read_a = false;
                }
            }
            if self.read_b {
                self.next_b = self.iter_b.next();
                if self.next_b == None {
                    self.read_b = false;
                }
            }
            match (&self.next_a, &self.next_b) {
                (None, None) => return None,
                (None, Some(b)) => {
                    self.read_a = false;
                    self.read_b = true;
                    match self.set_op {
                        SetOperation::And | SetOperation::Diff => {}
                        SetOperation::Or => return Some(b.clone()),
                    }
                }
                (Some(a), None) => {
                    self.read_a = true;
                    self.read_b = false;
                    match self.set_op {
                        SetOperation::And => {}
                        SetOperation::Or | SetOperation::Diff => return Some(a.clone()),
                    }
                }
                (Some(a), Some(b)) if a < b => {
                    self.read_a = true;
                    self.read_b = false;
                    match self.set_op {
                        SetOperation::And => {}
                        SetOperation::Or | SetOperation::Diff => return Some(a.clone()),
                    }
                }
                (Some(a), Some(b)) if a > b => {
                    self.read_a = false;
                    self.read_b = true;
                    match self.set_op {
                        SetOperation::And | SetOperation::Diff => {}
                        SetOperation::Or => return Some(b.clone()),
                    }
                }
                (Some(a), Some(_)) => {
                    self.read_a = true;
                    self.read_b = true;
                    match self.set_op {
                        SetOperation::Diff => {}
                        SetOperation::And | SetOperation::Or => return Some(a.clone()),
                    }
                }
            }
        }
    }
}
pub struct EntityIDIter {
    iter: Option<Iter<EntityID, bool>>,
}

impl Iterator for EntityIDIter {
    type Item = EntityID;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let (id, true) = self.iter.as_mut()?.next()? {
                return Some(id);
            }
        }
    }
}

fn to_json_value<T: serde::Serialize>(x: T) -> Result<serde_json::Value, String> {
    serde_json::to_value(x).map_err(|e| e.to_string())
}
fn to_field_values(value: serde_json::Value) -> Vec<FieldValue> {
    make_field_list(Vec::new(), value)
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct FieldValue {
    accessor: Vec<String>,
    value: Option<Value>,
}
impl FieldValue {
    pub fn new(accessor: Vec<String>, value: Option<Value>) -> FieldValue {
        FieldValue { accessor, value }
    }
    pub fn accessor_to_string(&self) -> String {
        format!(".{}", self.accessor.join("."))
    }
}

//TODO: for unknown reason. protobuf enum 0 does not show as field. only from 1 and on...
fn make_field_list(acessor: Vec<String>, json_value: serde_json::Value) -> Vec<FieldValue> {
    let mut values: Vec<FieldValue> = Vec::new();
    match json_value {
        serde_json::Value::Null => {
            values.push(FieldValue::new(acessor, Option::<Value>::None));
        }
        serde_json::Value::Bool(v) => {
            values.push(FieldValue::new(acessor, Some(Value::Boolean(v))));
        }
        serde_json::Value::Number(v) => match v.as_f64() {
            Some(v) => values.push(FieldValue::new(acessor, Some(Value::Number(v)))),
            None => values.push(FieldValue::new(acessor, Some(Value::Number(f64::NAN)))),
        },
        serde_json::Value::String(v) => values.push(FieldValue::new(acessor, Some(Value::Text(v)))),
        serde_json::Value::Array(a) => {
            for v in a {
                values.append(make_field_list(acessor.clone(), v).as_mut())
            }
        }
        serde_json::Value::Object(m) => {
            for (k, v) in m {
                let mut sub_acessor = acessor.clone();
                sub_acessor.push(k);
                values.append(make_field_list(sub_acessor, v).as_mut())
            }
        }
    }
    values
}

#[cfg(test)]
mod test_super {
    use super::*;
    use serde_json::{self, json};

    fn create_entity_a() -> Entity {
        Entity::Resource(
            "a".to_owned(),
            serde_json::from_value(json!({
                "name": "name",
                "description": "a description",
                "namespace": "namespace",
                "tags": ["a", "b", "c", "d"],
                "attributes": { "key1": "value1", "key2": "value2", "key3": ["val3a", "val3b", "val3c"] },
                "function":{
                    "inputs": [{
                        "name": "param1",
                        "description": "param1 description",
                        "parameterKind": "NUMBER",
                        "defaultValue": 6
                    },{
                        "name": "param2",
                        "description": "param2 description",
                        "parameterKind": "TEXT",
                        "defaultValue": "a text"
                    },{
                        "name": "param3",
                        "description": "param3 description",
                        "parameterKind": "PATH",
                        "defaultValue": "/path/to/dir"
                    },{
                        "name": "param4",
                        "description": "param4 description",
                        "parameterKind": "GENERIC",
                        "defaultValue": "generic"
                    }],
                    "outputs": [],
                    "baseCommand": ["echo", "hello", "world"],
                    "runtimeCapabilities":["ls", "cmd"]
                }
            })).unwrap(),
        )
    }
    fn create_entity_b() -> Entity {
        Entity::Resource(
            "b".to_owned(), //id has changed from a
            serde_json::from_value(json!({
                "name": "nameb", //name has changed from a
                "description": "another description", //description has changed from a
                "namespace": "namespace",
                "tags": ["a", "b", "c", "d", "e"], //tag has changed from a. add an 'e' tag.
                "attributes": { "key1": "value1", "key2": "value2", "key3": ["val3a", "val3b", "val3c"] },
                "function":{ //same type function as a
                    "inputs": [{
                        "name": "param1",
                        "description": "param1 description",
                        "parameterKind": "NUMBER",
                        "defaultValue": 6
                    },{
                        "name": "param2",
                        "description": "param2 description",
                        "parameterKind": "TEXT",
                        "defaultValue": "a text"
                    },{
                        "name": "param3",
                        "description": "param3 description",
                        "parameterKind": "PATH",
                        "defaultValue": "/path/to/dir"
                    },{
                        "name": "param4",
                        "description": "param4 description",
                        "parameterKind": "GENERIC",
                        "defaultValue": "generic"
                    }],
                    "outputs": [],
                    "baseCommand": ["echo", "hello", "world"],
                    "runtimeCapabilities":["ls", "cmd"]
                }
            })).unwrap(),
        )
    }
    fn create_entity_c() -> Entity {
        Entity::Resource(
            "c".to_owned(), //id has changed from a
            serde_json::from_value(json!({
                "name": "namec", //name has changed from a and b
                "description": "another description", //description has changed from a
                "namespace": "namespace",
                "tags": ["a", "b", "c", "d", "e"], //tag has changed from a. add an 'e' tag.
                "attributes": { "key1": "value1", "key2": "value2", "key3": ["val3a", "val3b", "val3c"] },
                "runtime":{ //type has changed from a and b
                    "container": {
                        "dockerfile": "MyDockerfile"
                    }
                }
            })).unwrap(),
        )
    }

    fn create_index(entities: Vec<&Entity>) -> Index {
        let mut index = new();
        for entity in entities {
            index.index(entity).unwrap();
        }
        index
    }

    #[test]
    fn test_with_kind() {
        let entity = create_entity_a();
        let index = create_index(vec![&entity]);
        let mut iter = index.with_kind(&entity.to_kind());
        assert_eq!(iter.next(), Some(entity.id()));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_with_field() {
        let entity = create_entity_a();
        let index = create_index(vec![&entity]);
        let ok = &Some(entity.id());
        let none = &None;
        let check = |field: &str,
                     fst: &Option<EntityID>,
                     snd: &Option<EntityID>,
                     trd: &Option<EntityID>| {
            let mut iter = index.with_field(&entity.to_kind(), field);
            assert_eq!(iter.next(), *fst, "with field: {}", field);
            assert_eq!(iter.next(), *snd, "with field: {}", field);
            assert_eq!(iter.next(), *trd, "with field: {}", field);
        };
        check(".name", ok, none, none);
        check(".attributes.key1", ok, none, none);
        check(".function.inputs.name", ok, none, none);
        check(".", ok, none, none);
        check("i dont exist", none, none, none);
    }

    #[test]
    fn test_with_value_eq() {
        let entity = create_entity_a();
        let index = create_index(vec![&entity]);
        let ok = &Some(entity.id());
        let none = &None;
        let check =
            |field: &str, value: &Option<Value>, fst: &Option<EntityID>, snd: &Option<EntityID>| {
                let mut iter = index.with_value_eq(&entity.to_kind(), field, value);
                assert_eq!(
                    iter.next(),
                    *fst,
                    "test for field:{} and value: {:?}",
                    field,
                    value
                );
                assert_eq!(
                    iter.next(),
                    *snd,
                    "test for field:{} and value: {:?}",
                    field,
                    value
                );
            };
        check(".name", &Some(Value::Text("name".to_string())), ok, none);
        check(
            ".description",
            &Some(Value::Text("a description".to_string())),
            ok,
            none,
        );
        check(
            ".function.baseCommand",
            &Some(Value::Text("echo".to_string())),
            ok,
            none,
        );
    }

    #[test]
    fn test_with_value_ne_3() {
        let ref a = create_entity_a();
        let ref b = create_entity_b();
        let ref c = create_entity_c();
        let index = create_index(vec![&c, &b, &a]);
        let oka = &Some(a.id());
        let okb = &Some(b.id());
        let okc = &Some(c.id());
        let none = &None;
        let check = |field: &str,
                     value: &Option<Value>,
                     fst: &Option<EntityID>,
                     snd: &Option<EntityID>,
                     trd: &Option<EntityID>| {
            let mut iter = index.with_value_ne(&a.to_kind(), field, value);
            assert_eq!(
                iter.next(),
                *fst,
                "test for field:{} and value: {:?}",
                field,
                value
            );
            assert_eq!(
                iter.next(),
                *snd,
                "test for field:{} and value: {:?}",
                field,
                value
            );
            assert_eq!(
                iter.next(),
                *trd,
                "test for field:{} and value: {:?}",
                field,
                value
            );
        };
        check(
            ".name",
            &Some(Value::Text("name".to_string())),
            okb,
            okc,
            none,
        );
        check(
            ".namespace",
            &Some(Value::Text("namespace".to_string())),
            none,
            none,
            none,
        );
        check(
            ".tags",
            &Some(Value::Text("e".to_string())),
            oka,
            none,
            none,
        );
        check(".tags", &Some(Value::Text("f".to_string())), oka, okb, okc);
        check(
            ".runtime.container.dockerfile",
            &Some(Value::Text("dockerfile".to_string())),
            okc,
            none, //a and b are functions. so this field is not defined for them
            none,
        );
    }

    #[test]
    fn test_with_value_lt() {
        let ref a = create_entity_a();
        let ref b = create_entity_b();
        let ref c = create_entity_c();
        let index = create_index(vec![&c, &b, &a]);
        let oka = &Some(a.id());
        let okb = &Some(b.id());
        let okc = &Some(c.id());
        let none = &None;

        let check = |field: &str,
                     value: &Option<Value>,
                     fst: &Option<EntityID>,
                     snd: &Option<EntityID>,
                     trd: &Option<EntityID>| {
            let mut iter = index.with_value_lt(&a.to_kind(), field, value);
            assert_eq!(
                iter.next(),
                *fst,
                "test for field:{} and value: {:?}",
                field,
                value
            );
            assert_eq!(
                iter.next(),
                *snd,
                "test for field:{} and value: {:?}",
                field,
                value
            );
            assert_eq!(
                iter.next(),
                *trd,
                "test for field:{} and value: {:?}",
                field,
                value
            );
        };
        check(
            ".tags",
            &Some(Value::Text("a".to_string())),
            none,
            none,
            none,
        );
        check(".tags", &Some(Value::Text("b".to_string())), oka, okb, okc);
    }

    #[test]
    fn test_with_value_lte() {
        let ref a = create_entity_a();
        let ref b = create_entity_b();
        let ref c = create_entity_c();
        let index = create_index(vec![&c, &b, &a]);
        let oka = &Some(a.id());
        let okb = &Some(b.id());
        let okc = &Some(c.id());
        let none = &None;
        let check = |field: &str,
                     value: &Option<Value>,
                     fst: &Option<EntityID>,
                     snd: &Option<EntityID>,
                     trd: &Option<EntityID>| {
            let mut iter = index.with_value_lte(&a.to_kind(), field, value);
            assert_eq!(
                iter.next(),
                *fst,
                "test for field:{} and value: {:?}",
                field,
                value
            );
            assert_eq!(
                iter.next(),
                *snd,
                "test for field:{} and value: {:?}",
                field,
                value
            );
            assert_eq!(
                iter.next(),
                *trd,
                "test for field:{} and value: {:?}",
                field,
                value
            );
        };
        check(
            ".tags",
            &Some(Value::Text("1".to_string())),
            none,
            none,
            none,
        );
        check(".tags", &Some(Value::Text("a".to_string())), oka, okb, okc);
        check(".tags", &Some(Value::Text("b".to_string())), oka, okb, okc);
    }

    #[test]
    fn test_with_value_gt() {
        let ref a = create_entity_a();
        let ref b = create_entity_b();
        let ref c = create_entity_c();
        let index = create_index(vec![&c, &b, &a]);
        let oka = &Some(a.id());
        let okb = &Some(b.id());
        let okc = &Some(c.id());
        let none = &None;

        let check = |field: &str,
                     value: &Option<Value>,
                     fst: &Option<EntityID>,
                     snd: &Option<EntityID>,
                     trd: &Option<EntityID>| {
            let mut iter = index.with_value_gt(&a.to_kind(), field, value);
            assert_eq!(
                iter.next(),
                *fst,
                "test for field:{} and value: {:?}",
                field,
                value
            );
            assert_eq!(
                iter.next(),
                *snd,
                "test for field:{} and value: {:?}",
                field,
                value
            );
            assert_eq!(
                iter.next(),
                *trd,
                "test for field:{} and value: {:?}",
                field,
                value
            );
        };
        check(".tags", &Some(Value::Text("d".to_string())), okb, okc, none);
        check(".tags", &Some(Value::Text("a".to_string())), oka, okb, okc);
    }

    #[test]
    fn test_with_value_gte() {
        let ref a = create_entity_a();
        let ref b = create_entity_b();
        let ref c = create_entity_c();
        let index = create_index(vec![&c, &b, &a]);
        let oka = &Some(a.id());
        let okb = &Some(b.id());
        let okc = &Some(c.id());
        let none = &None;

        let check = |field: &str,
                     value: &Option<Value>,
                     fst: &Option<EntityID>,
                     snd: &Option<EntityID>,
                     trd: &Option<EntityID>| {
            let mut iter = index.with_value_gte(&a.to_kind(), field, value);
            assert_eq!(
                iter.next(),
                *fst,
                "test for field:{} and value: {:?}",
                field,
                value
            );
            assert_eq!(
                iter.next(),
                *snd,
                "test for field:{} and value: {:?}",
                field,
                value
            );
            assert_eq!(
                iter.next(),
                *trd,
                "test for field:{} and value: {:?}",
                field,
                value
            );
        };
        check(".tags", &Some(Value::Text("e".to_string())), okb, okc, none);
        check(".tags", &Some(Value::Text("d".to_string())), oka, okb, okc);
        check(".tags", &Some(Value::Text("a".to_string())), oka, okb, okc);
    }

    #[test]
    fn test_with_value_eq_3() {
        let ref a = create_entity_a();
        let ref b = create_entity_b();
        let ref c = create_entity_c();
        let index = create_index(vec![&c, &b, &a]);
        let oka = &Some(a.id());
        let okb = &Some(b.id());
        let okc = &Some(c.id());
        let none = &None;
        let check = |field: &str,
                     value: &Option<Value>,
                     fst: &Option<EntityID>,
                     snd: &Option<EntityID>,
                     trd: &Option<EntityID>| {
            let mut iter = index.with_value_eq(&a.to_kind(), field, value);
            assert_eq!(
                iter.next(),
                *fst,
                "test for field:{} and value: {:?}",
                field,
                value
            );
            assert_eq!(
                iter.next(),
                *snd,
                "test for field:{} and value: {:?}",
                field,
                value
            );
            assert_eq!(
                iter.next(),
                *trd,
                "test for field:{} and value: {:?}",
                field,
                value
            );
        };
        check(
            ".name",
            &Some(Value::Text("name".to_string())),
            oka,
            none,
            none,
        );
        check(
            ".namespace",
            &Some(Value::Text("namespace".to_string())),
            oka,
            okb,
            okc,
        );
        check(".tags", &Some(Value::Text("e".to_string())), okb, okc, none);
        check(
            ".tags",
            &Some(Value::Text("f".to_string())),
            none,
            none,
            none,
        );
    }

    #[test]
    fn test_has_entity() {
        let ref entity = create_entity_a();
        let index = create_index(vec![entity]);
        assert_eq!(index.has_entity(entity), true);
    }
    #[test]
    fn test_has_field() {
        let ref entity = create_entity_a();
        let index = create_index(vec![entity]);
        assert_eq!(index.has_field(entity, ".name"), true);
        assert_eq!(index.has_field(entity, ".description"), true);
        assert_eq!(index.has_field(entity, ".namespace"), true);
        assert_eq!(index.has_field(entity, ".attributes"), true);
        assert_eq!(index.has_field(entity, ".attributes.key1"), true);
        assert_eq!(index.has_field(entity, ".attributes.key2"), true);
        assert_eq!(index.has_field(entity, ".attributes.key3"), true);
        assert_eq!(index.has_field(entity, ".function"), true);
        assert_eq!(index.has_field(entity, ".function.inputs"), true);
        assert_eq!(index.has_field(entity, ".function.inputs.name"), true);
        assert_eq!(index.has_field(entity, ".function.outputs"), false);
    }
    #[test]
    fn test_has_value() {
        let ref entity = create_entity_a();
        let index = create_index(vec![entity]);
        assert_eq!(
            index.has_field(entity, ".function.inputs.description"),
            true
        );
        assert_eq!(
            index.has_field(entity, ".function.inputs.parameterKind"),
            true
        );
        assert_eq!(
            index.has_field(entity, ".function.inputs.defaultValue"),
            true
        );
        assert_eq!(index.has_field(entity, ".function.baseCommand"), true);
        assert_eq!(
            index.has_field(entity, ".function.runtimeCapabilities"),
            true
        );

        assert_eq!(
            index.has_value(entity, ".name", &Some(Value::Text("name".to_string()))),
            true
        );
        assert_eq!(
            index.has_value(
                entity,
                ".description",
                &Some(Value::Text("a description".to_string()))
            ),
            true
        );
        assert_eq!(
            index.has_value(
                entity,
                ".namespace",
                &Some(Value::Text("namespace".to_string()))
            ),
            true
        );
        assert_eq!(
            index.has_value(entity, ".tags", &Some(Value::Text("a".to_string()))),
            true
        );
        assert_eq!(
            index.has_value(entity, ".tags", &Some(Value::Text("b".to_string()))),
            true
        );
        assert_eq!(
            index.has_value(entity, ".tags", &Some(Value::Text("c".to_string()))),
            true
        );
        assert_eq!(
            index.has_value(
                entity,
                ".function.inputs.name",
                &Some(Value::Text("param1".to_string()))
            ),
            true
        );
        assert_eq!(
            index.has_value(
                entity,
                ".function.inputs.description",
                &Some(Value::Text("param1 description".to_string()))
            ),
            true
        );
        assert_eq!(
            index.has_value(
                entity,
                ".function.inputs.defaultValue",
                &Some(Value::Number(6.0))
            ),
            true
        );
        assert_eq!(
            index.has_value(
                entity,
                ".function.inputs.parameterKind",
                &Some(Value::Text("NUMBER".to_string()))
            ),
            true
        );
        assert_eq!(
            index.has_value(
                entity,
                ".function.inputs.parameterKind",
                &Some(Value::Text("PATH".to_string()))
            ),
            true
        );
        assert_eq!(
            index.has_value(
                entity,
                ".function.inputs.parameterKind",
                &Some(Value::Text("TEXT".to_string()))
            ),
            true
        );
    }
}
