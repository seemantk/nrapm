extern crate log;
use rhai::{Module, Shared};
use crate::cmd;
use crate::cmd::nrkv;
use crate::rscript::rjson;

pub fn load_state_to_module(c: &cmd::Cli) -> Shared<Module> {
    log::debug!("Loading state into scripting engine");
    let vars = nrkv::values(&c);
    let mut res: Module = Module::new();
    for (key, value) in vars {
        log::debug!("rhai state: {} {:?}", key, value);
        res.set_var(key.as_str(), rjson::dynamic_from_value(&value));
    }
    Shared::new(res)
}

pub fn save_state(_c: &cmd::Cli, _m: *const Module) {
    log::debug!("Saving state from scripting engine");

}
