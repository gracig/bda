/*
use serde_json::Value;

pub trait Backend {}

pub struct Index<T: Backend> {
    backend: T,
}

pub fn new<T: Backend>(backend: T) -> Index<T> {
    Index::new(backend)
}

impl<T: Backend> Index<T> {
    pub fn new(backend: T) -> Self {
        Self { backend }
    }
    pub fn index(&self, ns: &str, id: &str, value: Value) -> Result<(), String> {
        Ok(())
    }
}
*/
/*
#[derive(Debug, Clone, PartialEq)]
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
fn make_field_list(acessor: Vec<String>, json_value: Value) -> Vec<FieldValue> {
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
*/
