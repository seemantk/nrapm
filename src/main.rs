use std;
pub mod cmd;
pub mod rscript;

fn main() {
    cmd::init();
    std::process::exit(0);
}
