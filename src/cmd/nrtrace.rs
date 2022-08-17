extern crate log;
use serde_json::{json, to_string, Map, Value};
use ureq::post;
use crate::cmd;

pub fn process_trace(c: &cmd::Cli, e: &String, s: &String, t: &String, i: &String, p: &String, n: &String, d: &u64, sampled: &bool, trace_type: &String, trace_category: &String, a: &Vec<String>) {
    log::trace!("NRAPM Trace() reached");
    let mut res = Map::new();
    let mut attr = Map::new();
    attr.insert("host".to_string(), Value::from(c.hostname.as_str()));
    res.insert("timestamp".to_string(), json!(c.timestamp));
    res.insert("trace.id".to_string(), Value::from(t.as_str()));
    res.insert("id".to_string(), Value::from(i.as_str()));
    if ! p.is_empty() {
        attr.insert("parent.id".to_string(), Value::from(p.as_str()));
    }
    attr.insert("name".to_string(), Value::from(n.as_str()));
    attr.insert("duration.ms".to_string(), json!(d));
    attr.insert("primary_application_id".to_string(), Value::from(format!("{}-{}", n, i)));
    attr.insert("service.name".to_string(), Value::from(s.as_str()));
    attr.insert("sampled".to_string(), json!(sampled));
    attr.insert("type".to_string(), Value::from(trace_type.as_str()));
    attr.insert("category".to_string(), Value::from(trace_category.as_str()));
    if ! e.is_empty() {
        attr.insert("error.message".to_string(), Value::from(e.as_str()));
    }
    for l in a {
        let pair: Vec<_> = l.splitn(2, "=").collect();
        if pair.len() != 2 {
            continue;
        }
        let key = pair[0];
        let value = pair[1];
        attr.insert(key.to_string(), cmd::string_to_value(&c, &c.eval, value));
    }
    res.insert("attributes".to_string(), json!(&attr));
    let out = json!([{
        "spans": [
            &res,
        ]
    }]);
    let payload = to_string(&out).unwrap();
    log::debug!("{}", &payload);
    send_trace(c, &payload);
}

pub fn process_trace_with_timestamp(c: &cmd::Cli, err: &String, ts: &u64,  s: &String, t: &String, i: &String, p: &String, n: &String, d: &u64, a: &Vec<String>) {
    log::trace!("NRAPM Trace() with timestamp {} reached", ts);
    let mut res = Map::new();
    let mut attr = Map::new();
    attr.insert("host".to_string(), Value::from(c.hostname.as_str()));
    res.insert("timestamp".to_string(), json!(ts));
    res.insert("trace.id".to_string(), Value::from(t.as_str()));
    res.insert("id".to_string(), Value::from(i.as_str()));
    if ! p.is_empty() {
        attr.insert("parent.id".to_string(), Value::from(p.as_str()));
    }
    attr.insert("name".to_string(), Value::from(n.as_str()));
    attr.insert("duration.ms".to_string(), json!(d));
    attr.insert("service.name".to_string(), Value::from(s.as_str()));
    if ! err.is_empty() {
        attr.insert("error.message".to_string(), Value::from(err.as_str()));
    }
    for l in a {
        let pair: Vec<_> = l.splitn(2, "=").collect();
        if pair.len() != 2 {
            continue;
        }
        let key = pair[0];
        let value = pair[1];
        attr.insert(key.to_string(), cmd::string_to_value(&c, &c.eval, value));
    }
    res.insert("attributes".to_string(), json!(&attr));
    let out = json!([{
        "spans": [
            &res,
        ]
    }]);
    let payload = to_string(&out).unwrap();
    log::debug!("{}", &payload);
    send_trace(c, &payload);
}

fn send_trace(c: &cmd::Cli, payload: &String) {
    let url = format!("https://{}/trace/v1", c.nr_trace);
    log::trace!("Endpoint URL: {}", url);
    let resp = post(&url)
        .set("Api-Key", &c.nr_insert)
        .set("Content-Type", "application/json")
        .set("Data-Format", "newrelic")
        .set("Data-Format-Version", "1")
        .send_bytes(payload.as_bytes()).unwrap();
    if resp.status() == 202 {
        log::debug!("Request was succesful");
        std::process::exit(0);
    } else {
        log::error!("Request failed");
        std::process::exit(1);
    }
}
