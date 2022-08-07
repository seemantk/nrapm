extern crate log;
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
