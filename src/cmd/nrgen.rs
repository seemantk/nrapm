extern crate log;
use uuid::{Uuid};
use crate::cmd;

pub fn process_generate(_c: &cmd::Cli) {
    log::trace!("NRAPM Gen() reached");
    println!("{}", generate_generate().simple().encode_lower(&mut Uuid::encode_buffer()));
}

pub fn generate_generate() -> Uuid {
    Uuid::new_v4()
}
