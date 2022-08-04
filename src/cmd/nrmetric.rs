extern crate log;
use crate::cmd;

pub fn process_metric(c: cmd::Cli) {
    log::trace!("NRCLI Metric() reached {}", c.hostname)
}
