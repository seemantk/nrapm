extern crate log;
use serde_json::{to_string,json, Map, Value};
use crate::cmd;

pub fn process_log(c: &cmd::Cli, t: &String, s: &String, a: &Vec<String>) {
    log::trace!("NRCLI Log() reached");
    for l in a {
        let mut res = Map::new();
        let mut attr = Map::new();
        attr.insert("host.name".to_string(), Value::from(c.hostname.as_str()));
        res.insert("timestamp".to_string(), json!(c.timestamp));
        attr.insert("logtype".to_string(), Value::from(t.as_str()));
        attr.insert("service".to_string(), Value::from(s.as_str()));
        res.insert("attributes".to_string(), json!(&attr));
        res.insert("message".to_string(), Value::from(l.as_str()));
        let payload = to_string(&res).unwrap();
        log::debug!("{}", &payload)
    }
}
