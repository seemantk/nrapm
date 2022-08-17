extern crate log;
use crate::cmd;
use crate::cmd::nrkv;
use std;

pub fn check_sanity(c: cmd::Cli) {
    log::trace!("NRAPM check_sanity() reached");
    if c.nr_account == "0" {
        log::error!("You did not specified New Relic account");
        std::process::exit(10)
    }
    log::debug!("NR Account is {}", c.nr_account);
    if c.nr_api.is_empty() {
        log::error!("You did not specified New Relic API key");
        std::process::exit(10)
    }
    log::debug!("NR API key is {}", c.nr_api);
    if c.nr_insert.is_empty() {
        log::error!("You did not specified New Relic INSERT key");
        std::process::exit(10)
    }
    if c.state.is_empty() {
        log::error!("You did not specified NRAPM state storage");
        std::process::exit(10)
    }
    log::debug!("NR INSERT key is {}", c.nr_insert);
    nrkv::open_kv(&c);
    log::trace!("NRAPM Sanity check is OK")
}
