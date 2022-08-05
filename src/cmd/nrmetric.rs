extern crate log;
use serde_json::{json,to_string, Value};
use crate::cmd;

pub fn process_metric(c: &cmd::Cli, n: &String, t: &String, v: &String, a: &Vec<String>) {
    log::trace!("NRCLI Metric() reached");
    let mut j = cmd::parse_args(false, &c.eval, &c.hostname, &c.timestamp, a.to_vec());
    j.insert("name".to_string(), Value::from(n.as_str()));
    j.insert("type".to_string(), Value::from(t.as_str()));
    j.insert("value".to_string(), cmd::string_to_value(&c.eval, v.as_str()));
    let out = json!(
        [{
            "metrics":[
                j,
            ]
        }]
    );
    let payload = to_string(&out).unwrap();
    log::debug!("{}", &payload)
}
