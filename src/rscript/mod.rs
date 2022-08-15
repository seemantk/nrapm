extern crate log;
use std;
use std::collections::HashMap;
use rhai::{Engine, Dynamic, EvalAltResult, Array, Map, Shared};
use serde_json::{json, Value};
use crate::cmd;

pub mod rjson;
mod rstate;

pub fn run_script(c: &cmd::Cli, s: &String) -> Value {
    let mut engine = Engine::new();
    let sm = rstate::load_state_to_module(&c);
    let state_mod = Shared::as_ptr(&sm);
    engine.register_global_module(sm);
    rstate::save_state(&c, state_mod);
    let res: Result<Dynamic, Box<EvalAltResult>> = engine.eval(s);
    match res {
        Ok(_) => {
            let value = res.unwrap();
            if value.is::<i64>() {
                let v = value.cast::<i64>();
                log::trace!("script returned <i64> = {}", &v);
                return Value::from(v);
            } else if value.is::<f64>() {
                let v = value.cast::<f64>();
                log::trace!("script returned <f64> = {}", &v);
                return Value::from(v);
            } else if value.is::<String>() {
                let v = value.cast::<String>();
                log::trace!("script returned <String> = {}", &v);
                return Value::from(v);
            } else if value.is::<bool>() {
                let v = value.cast::<bool>();
                log::trace!("script returned <bool> = {}", &v);
                return Value::from(v);
            } else if value.is::<Array>() {
                let v = value.cast::<Array>();
                log::trace!("script returned <Array> = {:?}", &v);
                let res: Vec<Value> = Vec::new();
                return json!(res);
            } else if value.is::<Map>() {
                let v = value.cast::<Map>();
                log::trace!("script returned <Map> = {:?}", &v);
                let res: HashMap<&str, Value> = HashMap::new();
                return json!(res);
            } else {
                log::error!("Script returned unexpected value: {:?}", value);
                std::process::exit(11);
            }
        }
        Err(err) => {
            log::error!("Evaluation error: {:?}", err);
            std::process::exit(10);
        }
    }
}
