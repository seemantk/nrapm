extern crate log;
use serde_json::{json, to_string, Map, Value};
use crate::cmd;

pub fn process_trace(c: &cmd::Cli, s: &String, t: &String, i: &String, p: &String, n: &String, d: &u64, a: &Vec<String>) {
    log::trace!("NRCLI Trace() reached");
    let mut res = Map::new();
    let mut attr = Map::new();
    attr.insert("host".to_string(), Value::from(c.hostname.as_str()));
    res.insert("timestamp".to_string(), json!(c.timestamp));
    res.insert("service".to_string(), Value::from(s.as_str()));
    res.insert("trace.id".to_string(), Value::from(t.as_str()));
    res.insert("id".to_string(), Value::from(i.as_str()));
    if ! p.is_empty() {
        attr.insert("parent.id".to_string(), Value::from(p.as_str()));
    }
    attr.insert("name".to_string(), Value::from(n.as_str()));
    attr.insert("duration.ms".to_string(), json!(d));
    for l in a {
        let pair: Vec<_> = l.splitn(2, "=").collect();
        if pair.len() != 2 {
            continue;
        }
        let key = pair[0];
        let value = pair[1];
        attr.insert(key.to_string(), cmd::string_to_value(value));
    }
    res.insert("attributes".to_string(), json!(&attr));
    let out = json!([{
        "spans": [
            &res,
        ]
    }]);
    let payload = to_string(&out).unwrap();
    log::debug!("{}", &payload);
}
