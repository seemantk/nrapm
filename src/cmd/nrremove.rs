extern crate log;
use crate::cmd;
use crate::cmd::nrkv;

pub fn process_remove(c: &cmd::Cli, a: &Vec<String>) {
    log::trace!("NRAPM Remove() reached");
    remove_from_args(&c, &a)
}

fn remove_from_args(c: &cmd::Cli, args: &Vec<String>) {
    for key in args {
        nrkv::delete(&c, &key);
    }
}
