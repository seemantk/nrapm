extern crate log;
use std::collections::HashMap;
use kv::*;
use crate::cmd;
use crate::rscript::rjson;
use serde_json::{Value};

pub fn open_kv(c: &cmd::Cli) -> Bucket<&str, Raw> {
    let cfg = Config::new(&c.state.as_str());
    let store = Store::new(cfg).unwrap();
    store.bucket::<&str, Raw>(Some(&c.nr_account.as_str())).unwrap()
}

pub fn store(c: &cmd::Cli, k: &str, v: &Value) {
    let bucket = open_kv(&c);
    let _ = bucket.set(&k, &rjson::from_value(&v));
    log::debug!("State: {} = {:?}", &k, &v);
}

pub fn values(c: &cmd::Cli) -> HashMap::<&str, &Value> {
    let res = HashMap::new();
    let bucket = open_kv(&c);
    for item in bucket.iter() {
        let i = item.unwrap();
        let key: String = i.key().unwrap();
        let value = i.value::<Raw>();
        let v = &rjson::to_value(&value.unwrap());
        log::debug!("state: {}, value: {:?}", &key, &v);
    }
    res
}
