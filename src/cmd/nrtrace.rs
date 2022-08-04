extern crate log;
use crate::cmd;

pub fn process_trace(c: cmd::Cli) {
    log::trace!("NRCLI Trace() reached {}", c.hostname)
}
