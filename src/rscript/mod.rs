extern crate log;
use std;
use rhai::{Engine, Dynamic, EvalAltResult};
use serde_json::{Value};

pub fn run_script(s: &String) -> Value {
    let engine = Engine::new();
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
