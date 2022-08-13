extern crate log;
use std::str;
use kv::Raw;
use serde_json::{from_str, Value};

pub fn from_value(v: &Value) -> Raw {
    Raw::from(v.to_string().as_str())
}

pub fn to_value(r: &Raw) -> Value {
    from_str(str::from_utf8(r.as_ref()).unwrap()).unwrap()
}
