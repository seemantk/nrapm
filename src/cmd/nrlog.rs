extern crate log;
use std::vec::Vec;
use serde_json::{to_string,json, Map, Value};
use ureq::post;
use crate::cmd;

pub fn process_log(c: &cmd::Cli, t: &String, s: &String, ti: &String, id: &String, a: &Vec<String>) {
    log::trace!("NRAPM Log() reached");
    let mut logs = Vec::new();
    for l in a {
        let mut res = Map::new();
        let mut attr = Map::new();
        attr.insert("host.name".to_string(), Value::from(c.hostname.as_str()));
        res.insert("timestamp".to_string(), json!(c.timestamp));
        attr.insert("logtype".to_string(), Value::from(t.as_str()));
        attr.insert("service".to_string(), Value::from(s.as_str()));
        if ! ti.is_empty() && ! id.is_empty() {
            attr.insert("trace.id".to_string(), Value::from(ti.as_str()));
            attr.insert("span.id".to_string(), Value::from(id.as_str()));

        }
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
    log::debug!("{}", &payload);
    send_log(c, &payload);
}

fn send_log(c: &cmd::Cli, payload: &String) {
    let url = format!("https://{}/log/v1", c.nr_log);
    log::trace!("Endpoint URL: {}", url);
    let resp = post(&url)
        .set("Api-Key", &c.nr_insert)
        .set("Content-Type", "application/json")
        .send_bytes(payload.as_bytes()).unwrap();
    if resp.status() == 202 {
        log::debug!("Request was succesful");
        std::process::exit(0);
    } else {
        log::error!("Request failed");
        std::process::exit(1);
    }
}
