extern crate log;
use std;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::{Value};
use crate::cmd;
use crate::cmd::nrtrace;
use crate::cmd::nrkv;


pub fn process_begin(c: &cmd::Cli, service: &String, trace_id: &String, id: &String, parent_id: &String, name: &String, instance_id: &String ) {
    log::trace!("NRAPM Begin() reached");
    let tx_id = transaction_key(&service, &trace_id, &id, &parent_id, &name, &instance_id);
    log::debug!("Transactionkey: {}", &tx_id);
    if nrkv::have(&c, &tx_id) {
        log::error!("Transaction {} has been already started", &tx_id);
        std::process::exit(15);
    }
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    nrkv::store(&c, &tx_id, &Value::from(stamp))
}

pub fn process_end(c: &cmd::Cli, error: &String, service: &String, trace_id: &String, id: &String, parent_id: &String, name: &String, sampled: &bool, trace_type: &String, trace_category: &String, instance_id: &String, a: &Vec<String>) {
    log::trace!("NRAPM End() reached");
    let tx_id = transaction_key(&service, &trace_id, &id, &parent_id, &name, &instance_id);
    log::debug!("Transactionkey: {}", &tx_id);
    if ! nrkv::have(&c, &tx_id) {
        log::error!("Transaction {} has not been started", &tx_id);
        std::process::exit(15);
    } else {
        log::debug!("Transaction {} has been found", &tx_id);
    }
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
    if ! instance_id.is_empty() {
        attr.insert("service.instance.id".to_string(), Value::from(instance_id.as_str()));
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
    nrkv::delete(&c, &tx_id)
}

fn transaction_key(service: &String, trace_id: &String, id: &String, parent_id: &String, name: &String, instance_id: &String) -> String {
    format!("{}-{}-{}-{}-{}-{}", &service, &trace_id, &id, &parent_id, &name, &instance_id)
}
