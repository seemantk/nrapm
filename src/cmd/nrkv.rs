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

pub fn have(c: &cmd::Cli, k: &str) -> bool {
    let bucket = open_kv(&c);
    bucket.contains(&k).unwrap()
}

pub fn delete(c: &cmd::Cli, k: &str) {
    let bucket = open_kv(&c);
    let _ = bucket.remove(&k);
    log::debug!("Remove from state: {}", &k);
}

pub fn values(c: &cmd::Cli) -> HashMap::<String, Value> {
    let mut res: HashMap<String, Value> = HashMap::new();
    let bucket = open_kv(&c);
    for item in bucket.iter() {
        let i = item.unwrap();
        let key: String = i.key().unwrap();
        let value = i.value::<Raw>();
        let v = rjson::to_value(&value.unwrap());
        log::debug!("state: {}, value: {:?}", &key, &v);
        res.insert(key, v);
    }
    return res;
}
