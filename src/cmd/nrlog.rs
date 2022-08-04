extern crate log;
use crate::cmd;

pub fn process_log(c: cmd::Cli) {
    log::trace!("NRCLI Log() reached {}", c.hostname)
}
