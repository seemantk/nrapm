extern crate log;
use std::vec::Vec;
use serde_json::{to_string,json, Map, Value};
use crate::cmd;

pub fn process_log(c: &cmd::Cli, t: &String, s: &String, a: &Vec<String>) {
    log::trace!("NRCLI Log() reached");
    let mut logs = Vec::new();
    for l in a {
        let mut res = Map::new();
        let mut attr = Map::new();
        attr.insert("host.name".to_string(), Value::from(c.hostname.as_str()));
        res.insert("timestamp".to_string(), json!(c.timestamp));
        attr.insert("logtype".to_string(), Value::from(t.as_str()));
        attr.insert("service".to_string(), Value::from(s.as_str()));
        res.insert("attributes".to_string(), json!(&attr));
        res.insert("message".to_string(), Value::from(l.as_str()));
        logs.push(res);
    }
    let out = json!(
        [{
            "logs":logs,
        }]
    );
    let payload = to_string(&out).unwrap();
    log::debug!("{}", &payload)
}
