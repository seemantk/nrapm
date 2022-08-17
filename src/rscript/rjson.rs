extern crate log;
use std::str::FromStr;
use rhai::{Dynamic, Scope};
use std::str;
use kv::Raw;
use serde_json::{json,from_str, Value};

pub fn from_value(v: &Value) -> Raw {
    Raw::from(v.to_string().as_str())
}

pub fn to_value(r: &Raw) -> Value {
    from_str(str::from_utf8(r.as_ref()).unwrap()).unwrap()
}

pub fn dynamic_from_value(v: &Value) -> Dynamic {
    if v.is_string() {
        Dynamic::from_str(v.as_str().unwrap()).unwrap()
    } else if v.is_i64() {
        Dynamic::from_int(v.as_i64().unwrap())
    } else if v.is_boolean() {
        Dynamic::from_bool(v.as_bool().unwrap())
    } else {
        Dynamic::from_bool(false)
    }
}

pub fn dynamic_to_value(k: &String, s: &mut Scope, v: &Value) -> Value {
    if v.is_string() {
        Value::from(s.get_value::<String>(k).unwrap())
    } else if v.is_i64() {
        Value::from(s.get_value::<i64>(k).unwrap())
    } else if v.is_boolean() {
        Value::from(s.get_value::<bool>(k).unwrap())
    } else {
        json!(null)
    }
}
