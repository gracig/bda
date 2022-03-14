use crate::data::{Entity, EntityID, EntityKind};
use bdaql::{Ast, Value};
use ordered_float::{Float, OrderedFloat};
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
    pub fn index(&self, entity: &Entity) -> Result<(), String> {
        match entity {
            Entity::Resource(_, r) => {
                self.update_indexes(&entity.id(), &to_field_values(to_json_value(r)?))
            }
        }
    }
    pub fn remove(&self, entity: &Entity) -> Result<(), String> {
        match entity {
            Entity::Resource(_, r) => {
                self.remove_indexes(&entity.id(), &to_field_values(to_json_value(r)?))
            }
        }
    }

    pub fn search(&self, kind: &EntityKind, ast: Box<Ast>) -> Box<dyn Iterator<Item = EntityID>> {
        match *ast {
            bdaql::Ast::Intersection(a, b) => {
                self.with_and(self.search(kind, a), self.search(kind, b))
            }
            bdaql::Ast::Union(a, b) => self.with_or(self.search(kind, a), self.search(kind, b)),
            bdaql::Ast::Difference(a, b) => {
                self.with_diff(self.search(kind, a), self.search(kind, b))
            }
            bdaql::Ast::Complement(a, b) => {
                self.with_diff(self.search(kind, b), self.search(kind, a))
            }
            bdaql::Ast::All => self.with_kind(kind),
            bdaql::Ast::Equal {
                fname,
                fvalue,
                negate,
            } => {
                let eq = self.with_value_eq(kind, &fname, &fvalue);
                if negate {
                    Box::new(EntityIDSetOperationIter::new(
                        SetOperation::Diff,
                        self.with_kind(kind),
                        eq,
                    ))
                } else {
                    eq
                }
            }
            bdaql::Ast::Defined { fname, negate } => {
                let defined = self.with_field(kind, &fname);
                if negate {
                    Box::new(EntityIDSetOperationIter::new(
                        SetOperation::Diff,
                        self.with_kind(kind),
                        defined,
                    ))
                } else {
                    defined
                }
            }
            bdaql::Ast::LessThan { fname, fvalue } => self.with_value_lt(kind, &fname, &fvalue),
            bdaql::Ast::LessThanOrEqual { fname, fvalue } => {
                self.with_value_lte(kind, &fname, &fvalue)
            }
            bdaql::Ast::GreaterThan { fname, fvalue } => self.with_value_gt(kind, &fname, &fvalue),
            bdaql::Ast::GreaterThanOrEqual { fname, fvalue } => {
                self.with_value_gte(kind, &fname, &fvalue)
            }
            bdaql::Ast::ContainsAll {
                fname,
                fvalues,
                negate,
            } => {
                let all = self.with_value_eq_all(kind, &fname, &fvalues);
                if negate {
                    Box::new(EntityIDSetOperationIter::new(
                        SetOperation::Diff,
                        self.with_field(kind, &fname),
                        all,
                    ))
                } else {
                    all
                }
            }
            bdaql::Ast::ContainsAny {
                fname,
                fvalues,
                negate,
            } => {
                let any = self.with_value_eq_any(kind, &fname, &fvalues);
                if negate {
                    Box::new(EntityIDSetOperationIter::new(
                        SetOperation::Diff,
                        self.with_field(kind, &fname),
                        any,
                    ))
                } else {
                    any
                }
            }
        }
    }

    pub fn values(
        &self,
        kind: &EntityKind,
        field: &str,
    ) -> Box<dyn Iterator<Item = Option<Value>>> {
        Box::new(ValueIter {
            iter: self
                .value_index
                .get(&(kind.to_owned(), field.to_string()))
                .ok()
                .and_then(|x| Some(x.0.iter()))
                .and_then(|r| r.ok()),
        })
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
    fn with_value_range<R>(
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
                        println!("Excluding value {:?}", v)
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

    pub fn with_value_eq_all(
        &self,
        kind: &EntityKind,
        field: &str,
        values: &Vec<Option<Value>>,
    ) -> Box<dyn Iterator<Item = EntityID>> {
        let mut stack = Vec::new();
        for value in values {
            stack.push(self.with_value_eq(kind, field, value));
            if stack.len() > 1 {
                let iter_a = stack.pop().unwrap();
                let iter_b = stack.pop().unwrap();
                stack.push(Box::new(EntityIDSetOperationIter::new(
                    SetOperation::And,
                    iter_a,
                    iter_b,
                )));
            }
        }
        match stack.pop() {
            Some(iter) => iter,
            None => Box::new(EntityIDIter { iter: None }),
        }
    }

    pub fn with_value_eq_any(
        &self,
        kind: &EntityKind,
        field: &str,
        values: &Vec<Option<Value>>,
    ) -> Box<dyn Iterator<Item = EntityID>> {
        let mut stack = Vec::new();
        for value in values {
            stack.push(self.with_value_eq(kind, field, value));
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
        match stack.pop() {
            Some(iter) => iter,
            None => Box::new(EntityIDIter { iter: None }),
        }
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

    fn update_indexes(&self, id: &EntityID, values: &Vec<FieldValue>) -> Result<(), String> {
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

    fn remove_indexes(&self, id: &EntityID, values: &Vec<FieldValue>) -> Result<(), String> {
        for v in values {
            //println!("{:?}:{:?}", v.accessor_to_string(), v.value);
            self.remove_world_index(id)
                .and(self.remove_value_index(id, &v.accessor_to_string(), &v.value))
                .and(self.remove_field_index(id, &v))?;
        }
        Ok(())
    }

    fn remove_world_index(&self, id: &EntityID) -> Result<Option<EntitySet>, String> {
        let id_set = self
            .world_index
            .get(&id.to_kind())
            .unwrap_or((OMap::new(), 0))
            .0;

        Ok(id_set
            .remove(id)
            .and(self.world_index.set(id.to_kind(), id_set))
            .map_err(|e| e.to_string())?)
    }

    fn remove_value_index(
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
            .remove(id)
            .and(v_set.set(value.clone(), id_set))
            .and(
                self.value_index
                    .set((id.to_kind(), field.to_string()), v_set),
            )
            .map_err(|e| e.to_string())?)
    }

    fn remove_field_index(&self, id: &EntityID, fv: &FieldValue) -> Result<(), String> {
        let mut cv = fv.clone();
        loop {
            let field = cv.accessor_to_string();
            let id_set = self
                .field_index
                .get(&(id.to_kind(), field.clone()))
                .unwrap_or((OMap::new(), 0))
                .0;
            id_set
                .remove(id)
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
            set_op,
            iter_a,
            iter_b,
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

pub struct ValueIter {
    iter: Option<Iter<Option<Value>, EntitySet>>,
}

impl Iterator for ValueIter {
    type Item = Option<Value>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.as_mut()?.next()?.0)
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
            Some(v) => values.push(FieldValue::new(
                acessor,
                Some(Value::Number(OrderedFloat(v))),
            )),
            None => values.push(FieldValue::new(
                acessor,
                Some(Value::Number(OrderedFloat::nan())),
            )),
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
            EntityID::ResourceID("a".to_owned()),
            serde_json::from_value(json!({
                "name": "name",
                "description": "a description",
                "namespace": "namespace",
                "tags": ["a", "b", "c", "d"],
                "attributes": { "key1": "value1", "key2": "value2", "key3": ["val3a", "val3b", "val3c"], "key4": null },
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
            EntityID::ResourceID("b".to_owned()), //id has changed from a
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
            EntityID::ResourceID("c".to_owned()), //id has changed from a
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
        let index = new();
        for entity in entities {
            index.index(entity).unwrap();
        }
        index
    }

    #[test]
    fn test_remove() {
        let ref entity = create_entity_a();
        let index = create_index(vec![&entity]);
        assert_eq!(index.has_entity(entity), true);
        assert_eq!(index.has_field(entity, ".attributes.key3"), true);
        assert_eq!(
            index.has_value(
                entity,
                ".description",
                &Some(Value::Text("a description".to_string()))
            ),
            true
        );
        index.remove(entity).unwrap();
        assert_eq!(index.has_entity(entity), false);
        assert_eq!(index.has_field(entity, ".attributes.key3"), false);
        assert_eq!(
            index.has_value(
                entity,
                ".description",
                &Some(Value::Text("a description".to_string()))
            ),
            false
        );
    }

    #[test]
    fn test_values() {
        let ref a = create_entity_a();
        let ref b = create_entity_b();
        let ref c = create_entity_c();
        let index = create_index(vec![a, b, c]);
        let mut iter = index.values(&a.to_kind(), ".tags");
        assert_eq!(iter.next(), Some(Some(Value::Text("a".to_string()))));
        assert_eq!(iter.next(), Some(Some(Value::Text("b".to_string()))));
        assert_eq!(iter.next(), Some(Some(Value::Text("c".to_string()))));
        assert_eq!(iter.next(), Some(Some(Value::Text("d".to_string()))));
        assert_eq!(iter.next(), Some(Some(Value::Text("e".to_string()))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_search() {
        let ref a = create_entity_a();
        let ref b = create_entity_b();
        let ref c = create_entity_c();
        let index = create_index(vec![a, b, c]);
        let oka = &Some(a.id());
        let okb = &Some(b.id());
        let okc = &Some(c.id());
        let none = &None;
        let check =
            |q: &str, fst: &Option<EntityID>, snd: &Option<EntityID>, trd: &Option<EntityID>| {
                let kind = &a.to_kind();
                let ast = Box::new(bdaql::from_str(q).unwrap());
                let mut iter = index.search(kind, ast);
                assert_eq!(iter.next(), *fst, "testing with query: {}", q);
                assert_eq!(iter.next(), *snd, "testing with query: {}", q);
                assert_eq!(iter.next(), *trd, "testing with query: {}", q);
                assert_eq!(iter.next(), None, "testing with query: {}", q);
            };
        check(".name is defined", oka, okb, okc);
        check(".name", oka, okb, okc);
        check(".name eq 'name'", oka, none, none);
        check(".name == 'nameb'", okb, none, none);
        check(".name=='namec'", okc, none, none);
        check(".name ne 'name'", okb, okc, none);
        check("!.name eq 'name'", okb, okc, none);
        check(".name!='name'", okb, okc, none);
        check(".name>'name'", okb, okc, none);
        check(".name gt 'name'", okb, okc, none);
        check(".name>='name'", oka, okb, okc);
        check(".name gte 'name'", oka, okb, okc);
        check(".name<'name'", none, none, none);
        check(".name lt 'name'", none, none, none);
        check(".name<='name'", oka, none, none);
        check(".name lte 'name'", oka, none, none);
        check(".name>'name' or .name=='name'", oka, okb, okc);
        check(".name>'name' and .name=='nameb'", okb, none, none);
        check(".tags@all['a','b','c','d']", oka, okb, okc);
        check(".tags@all['a','b','c','d','e']", okb, okc, none);
        check(".tags!@all['a','b','c','d','e']", oka, none, none);
        check(".tags@any['a','b','c','d','e']", oka, okb, okc);
        check(".tags!@any['a','b','c','d','e']", none, none, none);
        check(".attributes.key4 == null", oka, none, none);
        check(".attributes.key4 is nothing", oka, none, none);
        check(".attributes.key4 is nil", oka, none, none);
        check(".attributes.key4 is none", oka, none, none);
        check(".runtime", okc, none, none);
        check(".function", oka, okb, none);
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
                &Some(Value::Number(OrderedFloat(6.0)))
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
