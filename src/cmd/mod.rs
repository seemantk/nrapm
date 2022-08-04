extern crate log;
extern crate hostname;
use clap::{Args, Parser, Subcommand};
use std::env;
mod nrevt;
mod nrlog;
mod nrmetric;
mod nrtrace;
mod sanity;

#[derive(Parser, Clone)]
#[clap(name = "nrcli")]
#[clap(author = "Vladimir Ulogov <vulogov@newrelic.com>")]
#[clap(version = "1.0")]
#[clap(about = "CLI interface to a New Relic", long_about = None)]
pub struct Cli {
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

    #[clap(long, default_value_t = String::from(hostname::get().unwrap().into_string().unwrap()))]
    hostname: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Event(Event),
    Log(Log),
    Metric(Metric),
    Trace(Trace),
}

#[derive(Args, Clone)]
#[clap(about="Send Event to a New Relic")]
struct Event {
    #[clap(short, long, default_value_t = String::from("ShellEvent"))]
    evt_type: String,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone)]
#[clap(about="Send Logs to a New Relic")]
struct Log {
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone)]
#[clap(about="Send Metric to a New Relic")]
struct Metric {
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone)]
#[clap(about="Send Trace data to a New Relic")]
struct Trace {
    #[clap(last = true)]
    args: Vec<String>,
}

pub fn init() {
    let cli = Cli::parse();
    sanity::check_sanity(cli.clone());
    match &cli.command {
        Commands::Event(_) => {
            nrevt::process_event(cli);
        }
        Commands::Log(_) => {
            nrlog::process_log(cli);
        }
        Commands::Metric(_) => {
            nrmetric::process_metric(cli);
        }
        Commands::Trace(_) => {
            nrtrace::process_trace(cli);
        }
    }
}
