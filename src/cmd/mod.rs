extern crate log;
extern crate hostname;
extern crate flate2;
use clap::{Args, Parser, Subcommand};
use std::env;
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::{json, Map, Value};
use lexical_core;
use flate2::read::GzEncoder;
use flate2::Compression;
use std::io;
use std::io::prelude::*;
use std::os::unix::process::parent_id;
mod nrevt;
mod nrlog;
mod nrmetric;
mod nrtrace;
mod nreval;
mod sanity;
mod eval;

#[derive(Parser, Clone)]
#[clap(name = "nrcli")]
#[clap(author = "Vladimir Ulogov <vulogov@newrelic.com>")]
#[clap(version = "1.0")]
#[clap(about = "CLI interface to a New Relic", long_about = None)]
pub struct Cli {
    #[clap(short, action = clap::ArgAction::Count)]
    eval: u8,

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

    #[clap(long, default_value_t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())]
    timestamp: u64,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Eval(Eval),
    Event(Event),
    Log(Log),
    Metric(Metric),
    Trace(Trace),
    Process(Process),
}

#[derive(Args, Clone, Debug)]
#[clap(about="Evaluate expressions")]
struct Eval {
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Send Event to a New Relic")]
struct Event {
    #[clap(short, long, default_value_t = String::from("ShellEvent"))]
    evt_type: String,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Send information about process to a New Relic")]
struct Process {
    #[clap(short, long, default_value_t = String::from("ShellEvent"))]
    evt_type: String,

    #[clap(short, long, default_value_t = parent_id().try_into().unwrap())]
    pid: i32,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Send Logs to a New Relic")]
struct Log {
    #[clap(last = true)]
    args: Vec<String>,

    #[clap(short, long, default_value_t = String::from("syslog"))]
    log_type: String,

    #[clap(short, long, default_value_t = String::from("shell"))]
    service: String,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Send Metric to a New Relic")]
struct Metric {
    #[clap(short, long, required=true)]
    name: String,

    #[clap(short, long, default_value_t = String::from("gauge"))]
    metric_type: String,

    #[clap(short, long, required=true)]
    value: String,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Send Trace data to a New Relic")]
struct Trace {
    #[clap(last = true)]
    args: Vec<String>,

    #[clap(short, long, default_value_t = String::from("shell"))]
    service: String,

    #[clap(short, long, required=true)]
    trace_id: String,

    #[clap(short, long, required=true)]
    id: String,

    #[clap(short, long, default_value_t = String::from(""))]
    parent_id: String,

    #[clap(short, long, required=true)]
    name: String,

    #[clap(short, long, required=true)]
    duration: u64,


}

pub fn init() {
    let cli = Cli::parse();
    sanity::check_sanity(cli.clone());
    match &cli.command {
        Commands::Eval(eval) => {
            log::debug!("Evaluation mode is {}", cli.eval);
            if cli.eval > 0 {
                nreval::eval_expression(&cli, &eval.args);
            } else {
                log::error!("Evaluation mode is off");
            }
        }
        Commands::Event(event) => {
            nrevt::process_event(&cli, &event.evt_type, &event.args);
        }
        Commands::Log(nrlog) => {
            nrlog::process_log(&cli, &nrlog.log_type, &nrlog.service, &nrlog.args);
        }
        Commands::Metric(met) => {
            nrmetric::process_metric(&cli, &met.name, &met.metric_type, &met.value, &met.args);
        }
        Commands::Trace(trace) => {
            nrtrace::process_trace(&cli, &trace.service, &trace.trace_id, &trace.id, &trace.parent_id, &trace.name, &trace.duration, &trace.args);
        }
        Commands::Process(proc) => {
            nrevt::process_process_event(&cli, &proc.evt_type, &proc.pid, &proc.args);
        }
    }
}



pub fn parse_args(d: bool, e: &u8, h: &String, t: &u64, args: Vec<String>) -> Map<String, Value> {
    log::trace!("Converting arguments to a JSON value. Eval mode is {}", e);
    let mut res = Map::new();
    let mut attr = Map::new();
    attr.insert("host.name".to_string(), Value::from(h.as_str()));
    res.insert("timestamp".to_string(), json!(t));
    for a in args {
        let pair: Vec<_> = a.splitn(2, "=").collect();
        if pair.len() != 2 {
            continue;
        }
        let key = pair[0];
        let value = pair[1];
        if d {
            res.insert(key.to_string(), string_to_value(e, value));
        } else {
            attr.insert(key.to_string(), string_to_value(e, value));
        }
        if d {
            log::trace!("Setting values {} {}", key, value);
        } else {
            log::trace!("Setting attributes {} {}", key, value);
        }
    }
    res.insert("attributes".to_string(), json!(&attr));
    res
}

pub fn string_to_value(e: &u8, v: &str) -> Value {
    if *e > 0 {
        let r = eval::eval_expression(v);
        match r {
            Ok(res) => {
                if res.is_string() {
                    return Value::from(res.as_string().unwrap());
                } else if res.is_int() {
                    return Value::from(res.as_int().unwrap());
                } else if res.is_float() {
                    return Value::from(res.as_float().unwrap());
                } else if res.is_boolean() {
                    return Value::from(res.as_boolean().unwrap());
                } else {
                    log::trace!("{:?}", res);
                    return json!(null);
                }
            }
            Err(_) => {
                return Value::from(v);
            }
        }
    }
    match v {
        "true" => { return json!(true); }
        "false" => { return json!(false); }
        _=> {
            if v.chars().nth(0).unwrap() == '\"' {
                return Value::from(&v[1..v.len()-1]);
            }
            let r: Result<i64, _> = lexical_core::parse(v.as_bytes());
            match r {
                Ok(val) => { return Value::from(val); }
                Err(_) => {
                    let r: Result<f64, _> = lexical_core::parse(v.as_bytes());
                    match r {
                        Ok(val) => { return Value::from(val); }
                        Err(_) => { return Value::from(v); }
                    }
                }
            }
        }
    }
}

pub fn compress_payload(payload: &String) -> io::Result<Vec<u8>> {
    let mut result = Vec::new();
    let mut z = GzEncoder::new(&payload.as_bytes()[..], Compression::fast());
    z.read_to_end(&mut result)?;
    Ok(result)
}
