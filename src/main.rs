extern crate log;
use env_logger::Env;
use std;
pub mod cmd;
pub mod rscript;

fn main() {
    let env = Env::default()
        .filter_or("NRAPM_LOG_LEVEL", "error")
        .write_style_or("NRAPM_LOG_STYLE", "always");

    env_logger::init_from_env(env);
    log::trace!("NRAPM main() function is reached");
    cmd::init();
    std::process::exit(0);
}
