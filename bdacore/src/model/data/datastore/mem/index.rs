use bdaql::Value;
use ppom::mdb::OMap;

use crate::model::data::{Entity, EntityID, EntityKind};

#[cfg(test)]
use serde_json::{self, json};

type FieldName = String;
type WorldIndex = OMap<(EntityKind, EntityID), bool>;
type FieldIndex = OMap<(EntityKind, FieldName, EntityID), bool>;
type ValueIndex = OMap<(EntityKind, FieldName, Option<Value>, EntityID), bool>;

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
                self.create_index(&entity.id(), &to_field_values(to_json_value(r)?))
            }
        }
    }
    pub fn has_entity(&self, entity: &Entity) -> bool {
        self.world_index
            .get(&(entity.to_kind(), entity.id()))
            .unwrap_or((false, 0))
            .0
    }
    pub fn has_field(&self, entity: &Entity, field: &str) -> bool {
        self.field_index
            .get(&(entity.to_kind(), field.to_string(), entity.id()))
            .unwrap_or((false, 0))
            .0
    }
    pub fn has_value(&self, entity: &Entity, field: &str, value: &Option<Value>) -> bool {
        self.value_index
            .get(&(
                entity.to_kind(),
                field.to_string(),
                value.to_owned(),
                entity.id(),
            ))
            .unwrap_or((false, 0))
            .0
    }

    fn create_index(&mut self, id: &EntityID, values: &Vec<FieldValue>) -> Result<(), String> {
        for v in values {
            //println!("{:?}:{:?}", v.accessor_to_string(), v.value);
            let world_index = (id.to_kind(), id.clone());
            let value_index = (
                id.to_kind(),
                v.accessor_to_string(),
                v.value.clone(),
                id.clone(),
            );
            //update world index to be used on [ALL] queries
            self.world_index
                .set(world_index.to_owned(), true)
                .map_err(|op| op.to_string())?;

            //Update value index to be used on ord and eq (==,!=, <, <=, >, >=) queries
            self.value_index
                .set(value_index.to_owned(), true)
                .map_err(|op| op.to_string())?;

            //Update field index to be used on defined predicates
            let mut cv = v.clone();
            loop {
                let field_index = (id.to_kind(), cv.accessor_to_string(), id.clone());
                self.field_index
                    .set(field_index, true)
                    .map_err(|op| op.to_string())?;
                if cv.accessor.is_empty() {
                    break;
                }
                cv.accessor.pop();
            }
        }
        Ok(())
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

#[test]
fn test_create_index() {
    let mut index = new();
    let entity = Entity::Resource(
        "id".to_owned(),
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
    );

    index.index(&entity).unwrap();
    assert_eq!(index.has_entity(&entity), true);
    assert_eq!(index.has_field(&entity, ".name"), true);
    assert_eq!(index.has_field(&entity, ".description"), true);
    assert_eq!(index.has_field(&entity, ".namespace"), true);
    assert_eq!(index.has_field(&entity, ".attributes"), true);
    assert_eq!(index.has_field(&entity, ".attributes.key1"), true);
    assert_eq!(index.has_field(&entity, ".attributes.key2"), true);
    assert_eq!(index.has_field(&entity, ".attributes.key3"), true);
    assert_eq!(index.has_field(&entity, ".function"), true);
    assert_eq!(index.has_field(&entity, ".function.inputs"), true);
    assert_eq!(index.has_field(&entity, ".function.inputs.name"), true);
    assert_eq!(index.has_field(&entity, ".function.outputs"), false);
    assert_eq!(
        index.has_field(&entity, ".function.inputs.description"),
        true
    );
    assert_eq!(
        index.has_field(&entity, ".function.inputs.parameterKind"),
        true
    );
    assert_eq!(
        index.has_field(&entity, ".function.inputs.defaultValue"),
        true
    );
    assert_eq!(index.has_field(&entity, ".function.baseCommand"), true);
    assert_eq!(
        index.has_field(&entity, ".function.runtimeCapabilities"),
        true
    );

    assert_eq!(
        index.has_value(&entity, ".name", &Some(Value::Text("name".to_string()))),
        true
    );
    assert_eq!(
        index.has_value(
            &entity,
            ".description",
            &Some(Value::Text("a description".to_string()))
        ),
        true
    );
    assert_eq!(
        index.has_value(
            &entity,
            ".namespace",
            &Some(Value::Text("namespace".to_string()))
        ),
        true
    );
    assert_eq!(
        index.has_value(&entity, ".tags", &Some(Value::Text("a".to_string()))),
        true
    );
    assert_eq!(
        index.has_value(&entity, ".tags", &Some(Value::Text("b".to_string()))),
        true
    );
    assert_eq!(
        index.has_value(&entity, ".tags", &Some(Value::Text("c".to_string()))),
        true
    );
    assert_eq!(
        index.has_value(
            &entity,
            ".function.inputs.name",
            &Some(Value::Text("param1".to_string()))
        ),
        true
    );
    assert_eq!(
        index.has_value(
            &entity,
            ".function.inputs.description",
            &Some(Value::Text("param1 description".to_string()))
        ),
        true
    );
    assert_eq!(
        index.has_value(
            &entity,
            ".function.inputs.defaultValue",
            &Some(Value::Number(6.0))
        ),
        true
    );
    assert_eq!(
        index.has_value(
            &entity,
            ".function.inputs.parameterKind",
            &Some(Value::Text("NUMBER".to_string()))
        ),
        true
    );
    assert_eq!(
        index.has_value(
            &entity,
            ".function.inputs.parameterKind",
            &Some(Value::Text("PATH".to_string()))
        ),
        true
    );
    assert_eq!(
        index.has_value(
            &entity,
            ".function.inputs.parameterKind",
            &Some(Value::Text("TEXT".to_string()))
        ),
        true
    );
}
