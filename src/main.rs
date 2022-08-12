extern crate log;
use env_logger::Env;
use std;
pub mod cmd;
pub mod rscript;

fn main() {
    let env = Env::default()
        .filter_or("NRCLI_LOG_LEVEL", "trace")
        .write_style_or("NRCLI_LOG_STYLE", "always");

    env_logger::init_from_env(env);
    log::trace!("NRCLI main() function is reached");
    cmd::init();
    std::process::exit(0);
}
