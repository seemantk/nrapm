extern crate log;
use sysinfo::{Pid, ProcessExt, System, SystemExt};
use serde_json::{json,to_string, Value};
use ureq::post;
use crate::cmd;
use std;

pub fn process_event(c: &cmd::Cli, t: &String, a: &Vec<String>) {
    log::trace!("NRCLI Event() reached");
    let mut j = cmd::parse_args(true, &c.eval, &c.hostname, &c.timestamp, a.to_vec());
    j.insert("eventType".to_string(), Value::from(t.as_str()));
    let out = json!(
        [j,]
    );
    let payload = to_string(&out).unwrap();
    log::debug!("{}", &payload);
    send_event(c, &payload);
}

pub fn process_process_event(c: &cmd::Cli, t: &String, p: &i32, a: &Vec<String>) {
    log::trace!("NRCLI Event() reached");
    let mut sys = System::new();
    sys.refresh_all();
    let mut j = cmd::parse_args(true, &c.eval, &c.hostname, &c.timestamp, a.to_vec());
    j.insert("eventType".to_string(), Value::from(t.as_str()));
    j.insert("sysname".to_string(), Value::from(sys.name()));
    j.insert("kernel".to_string(), Value::from(sys.kernel_version()));
    j.insert("osversion".to_string(), Value::from(sys.os_version()));
    j.insert("hostname".to_string(), Value::from(sys.host_name()));
    let parent = &sys.processes()[&Pid::from(*p)];
    let disk_usage = parent.disk_usage();
    j.insert("name".to_string(), Value::from(parent.name()));
    j.insert("memory".to_string(), Value::from(parent.memory()*1024));
    j.insert("virtualmemory".to_string(), Value::from(parent.virtual_memory()*1024));
    j.insert("cpu".to_string(), Value::from(parent.cpu_usage()));
    j.insert("disk_bytes_read".to_string(), Value::from(disk_usage.read_bytes));
    j.insert("disk_bytes_write".to_string(), Value::from(disk_usage.written_bytes));
    let out = json!(
        [j,]
    );
    let payload = to_string(&out).unwrap();
    log::debug!("{}", &payload);
    send_event(c, &payload);
}

fn send_event(c: &cmd::Cli, payload: &String) {
    let zpayload = cmd::compress_payload(payload).unwrap();
    let zp: &[u8] = &zpayload;
    let url = format!("https://{}/v1/accounts/{}/events", c.nr_event, c.nr_account);
    log::trace!("Endpoint URL: {}", url);
    let resp = post(&url)
        .set("Content-Encoding", "gzip")
        .set("Api-Key", &c.nr_insert)
        .set("Content-Type", "application/json")
        .send_bytes(zp).unwrap();
    if resp.status() == 200 {
        log::debug!("Request was succesful");
        std::process::exit(0);
    } else {
        log::error!("Request failed");
        std::process::exit(1);
    }
}
