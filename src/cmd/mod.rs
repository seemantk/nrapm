extern crate log;
use clap::{Args, Parser, Subcommand};
use std::env;

#[derive(Parser)]
#[clap(name = "nrcli")]
#[clap(author = "Vladimir Ulogov <vulogov@newrelic.com>")]
#[clap(version = "1.0")]
#[clap(about = "CLI interface to a New Relic", long_about = None)]
struct Cli {
    #[clap(long, default_value_t = String::from("insights-collector.newrelic.com"))]
    nr_event: String,

    #[clap(long, default_value_t = String::from("metric-api.newrelic.com"))]
    nr_metric: String,

    #[clap(long, default_value_t = String::from("log-api.newrelic.com"))]
    nr_log: String,

    #[clap(long, default_value_t = String::from("trace-api.newrelic.com"))]
    nr_trace: String,

    #[clap(long, default_value_t = String::from(env::var("NEWRELIC_ACCOUNT").unwrap_or("0".to_string())))]
    nr_account: String,

    #[clap(long, default_value_t = String::from(env::var("NEWRELIC_API").unwrap_or("".to_string())))]
    nr_api: String,

    #[clap(long, default_value_t = String::from(env::var("NEWRELIC_INSERTKEY").unwrap_or("".to_string())))]
    nr_insert: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Send(Send),
}

#[derive(Args)]
struct Send {
    #[clap(value_parser, forbid_empty_values = true)]
    fname: Option<String>,
}


pub fn init() {
    let cli = Cli::parse();
    log::debug!("NR accunt: {}", cli.nr_account);
    match &cli.command {
        Commands::Send(fname) => {
            log::debug!("'nrcli add' was used, name is: {:?}", fname.fname)
        }
    }
}
