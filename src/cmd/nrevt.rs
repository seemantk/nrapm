extern crate log;
use serde_json::{json,to_string};
use crate::cmd;

pub fn process_event(c: &cmd::Cli, a: &Vec<String>) {
    log::trace!("NRCLI Event() reached");
    let j = cmd::parse_args(true, &c.eval, &c.hostname, &c.timestamp, a.to_vec());
    let out = json!(
        [j,]
    );
    let payload = to_string(&out).unwrap();
    log::debug!("{}", &payload)
}
