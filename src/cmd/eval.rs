use crate::rscript;
use serde_json::{Value};
use crate::cmd;

pub fn eval_expression(c: &cmd::Cli, e: &str) -> Value {
    rscript::run_script(&c, &e.to_string())
}
