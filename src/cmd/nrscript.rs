
use std::io::{self, BufRead};
use serde_json::{Value};
use crate::rscript;
use crate::cmd;

fn read_from_stdin() -> io::Result<String> {
    let mut lines = io::stdin().lock().lines();
    let mut user_input = String::new();

    while let Some(line) = lines.next() {
        let last_input = line?;

        // stop reading
        if last_input.len() == 0 {
            break;
        }

        // add a new line once user_input starts storing user input
        if user_input.len() > 0 {
            user_input.push_str("\n");
        }

        // store user input
        user_input.push_str(&last_input);
    }
    Ok(user_input)
}

pub fn eval_expression(_c: &cmd::Cli, a: &Vec<String>)  {
    let mut res: Value;
    for s in a {
        if s == "-" {
            let s = read_from_stdin().unwrap();
            res = rscript::run_script(&s);
        } else {
            res = rscript::run_script(&s);
        }
        println!("{}", res);
    }
}
