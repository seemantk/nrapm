use crate::rscript;
use serde_json::{Value};

pub fn eval_expression(e: &str) -> Value {
    rscript::run_script(&e.to_string())
}
