use bdaql::{Rational, Value as BValue};
use serde::{Deserialize, Serialize};
use serde_json::Value as JValue;
use std::{error::Error, fmt::Debug, sync::Arc};

pub trait Backend {
    fn txwrite(&self) -> Result<Box<dyn TxWrite>, Box<dyn Error>>;
    fn add<'t, 'k, 'v, T: ?Sized, K, V>(
        &self,
        tx: &'t T,
        key: &'k K,
        value: &'v V,
    ) -> Result<(), Box<dyn Error>>
    where
        T: TxWrite,
        K: Debug + PartialEq<K> + Serialize + Deserialize<'k>,
        V: Debug + PartialEq<V> + Serialize + Deserialize<'v>;
}
pub trait TxWrite {
    fn abort(&self) -> Result<(), Box<dyn Error>>;
    fn commit(&self) -> Result<(), Box<dyn Error>>;
}
pub trait TxRead {
    fn abort(&self) -> Result<(), Box<dyn Error>>;
    fn commit(&self) -> Result<(), Box<dyn Error>>;
}

pub struct Index<T: Backend> {
    name: String,
    backend: Arc<T>,
}
pub fn new<T: Backend>(name: &str, backend: Arc<T>) -> Index<T> {
    Index::new(name, backend)
}

impl<T: Backend> Index<T> {
    pub fn new(name: &str, backend: Arc<T>) -> Self {
        Self {
            name: name.to_string(),
            backend,
        }
    }
    pub fn index<'k, 'v, K, V>(&self, id: &'k K, data: &'v V) -> Result<(), Box<dyn Error>>
    where
        K: Debug + PartialEq<K> + Serialize + Ord + Eq + Deserialize<'k>,
        V: Debug + PartialEq<V> + Serialize + Ord + Eq + Deserialize<'v>,
    {
        let tx = self.backend.txwrite()?;
        let tx = tx.as_ref();
        for mut fv in flatten(Vec::new(), serde_json::to_value(data)?) {
            self.backend.add(tx.into(), &self.v_key(&fv), id)?;
            while !fv.field.is_empty() {
                self.backend.add(tx.into(), &self.f_key(&fv), id)?;
                fv.field.pop();
            }
            self.backend.add(tx.into(), &self.f_key(&fv), id)?;
        }
        tx.commit()
    }
    fn f_key(&self, fv: &FieldValue) -> (BValue, BValue) {
        (BValue::Text(self.name.clone()), BValue::Text(fv.field()))
    }
    fn v_key(&self, fv: &FieldValue) -> (BValue, BValue, Option<BValue>) {
        (
            BValue::Text(self.name.clone()),
            BValue::Text(fv.field()),
            fv.value.clone(),
        )
    }
}

fn flatten(field: Vec<String>, json_value: JValue) -> Vec<FieldValue> {
    let mut values: Vec<FieldValue> = Vec::new();
    match json_value {
        serde_json::Value::Null => {
            values.push(FieldValue {
                field,
                value: Option::<BValue>::None,
            });
        }
        serde_json::Value::Bool(v) => {
            values.push(FieldValue {
                field,
                value: Some(BValue::Boolean(v)),
            });
        }
        serde_json::Value::Number(v) => match v.as_f64() {
            Some(v) => values.push(FieldValue {
                field,
                value: Some(BValue::Rational(Rational::from(v))),
            }),
            None => values.push(FieldValue {
                field,
                value: Some(BValue::Rational(Rational::from(f64::NAN))),
            }),
        },
        serde_json::Value::String(v) => values.push(FieldValue {
            field,
            value: Some(BValue::Text(v)),
        }),
        serde_json::Value::Array(a) => {
            for v in a {
                values.append(flatten(field.clone(), v).as_mut())
            }
        }
        serde_json::Value::Object(m) => {
            for (k, v) in m {
                let mut sub_field = field.clone();
                sub_field.push(k);
                values.append(flatten(sub_field, v).as_mut())
            }
        }
    }
    values
}
#[derive(Debug, Clone, PartialEq)]
struct FieldValue {
    field: Vec<String>,
    value: Option<BValue>,
}
impl FieldValue {
    fn field(&self) -> String {
        format!(".{}", self.field.join("."))
    }
}
