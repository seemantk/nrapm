extern crate log;
use crate::cmd;
use crate::cmd::nrkv;

pub fn process_set(c: &cmd::Cli, a: &Vec<String>) {
    log::trace!("NRCLI Set() reached");
    set_from_args(&c, &a)
}

fn set_from_args(c: &cmd::Cli, args: &Vec<String>) {
    for a in args {
        let pair: Vec<_> = a.splitn(2, "=").collect();
        if pair.len() != 2 {
            continue;
        }
        let key = pair[0];
        let value = pair[1];
        let v = cmd::raw_string_to_value(&c, &value);
        nrkv::store(&c, &key, &v);
    }
}
