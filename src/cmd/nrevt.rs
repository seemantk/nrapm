extern crate log;
use serde_json::{to_string};
use crate::cmd;

pub fn process_event(c: &cmd::Cli, a: &Vec<String>) {
    log::trace!("NRCLI Event() reached");
    let j = cmd::parse_args(true, &c.hostname, &c.timestamp, a.to_vec());
    let payload = to_string(&j).unwrap();
    log::debug!("{}", payload)
}
