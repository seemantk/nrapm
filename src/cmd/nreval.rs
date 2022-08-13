extern crate log;
use serde_json::{to_string};
use crate::cmd;

pub fn eval_expression(c: &cmd::Cli, a: &Vec<String>) {
    log::trace!("NRCLI Eval() reached");
    for e in a {
        let res = cmd::string_to_value(&c, &c.eval, e);
        println!("{:?}", to_string(&res).unwrap());
    }
}
