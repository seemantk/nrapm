extern crate log;
use crate::cmd;

pub fn process_event(c: cmd::Cli) {
    log::trace!("NRCLI Event() reached {}", c.hostname)
}
