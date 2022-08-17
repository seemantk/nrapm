extern crate log;
use rhai::{Scope};
use crate::cmd;
use crate::cmd::nrkv;
use crate::rscript::rjson;

pub fn load_state(c: &cmd::Cli, s: &mut Scope) {
    log::debug!("Loading state into scripting engine");
    let vars = nrkv::values(&c);
    for (key, value) in &vars {
        let v = rjson::dynamic_from_value(&value);
        s.set_value(key, v);
    }
}

pub fn save_state(c: &cmd::Cli, s: &mut Scope) {
    log::debug!("Saving state from scripting engine");
    let vars = nrkv::values(&c);
    for (key, value) in &vars {
        let v = rjson::dynamic_to_value(key, s, value);
        nrkv::store(&c, key, &v);
    }
}
