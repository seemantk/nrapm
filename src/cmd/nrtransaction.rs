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
    
    nrkv::delete(&c, &tx_id)
}

fn transaction_key(service: &String, trace_id: &String, id: &String, parent_id: &String, name: &String, instance_id: &String) -> String {
    format!("{}-{}-{}-{}-{}-{}", &service, &trace_id, &id, &parent_id, &name, &instance_id)
}
