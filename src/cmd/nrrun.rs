extern crate log;
use std::time::{SystemTime, UNIX_EPOCH};
use std;
use std::{thread, time};
use sysinfo::{Pid, System, SystemExt};
use crate::cmd;
use crate::cmd::nrevt;
use crate::cmd::nrtrace;


pub fn process_run(c: &cmd::Cli, s: &String, t: &String,  e: &String, i: &String, p: &String, a: &Vec<String>) {
    if a.len() < 1 {
        log::error!("You did not pass any information about command and command arguments");
        std::process::exit(13)
    }
    let cmdname = &a[0];
    let cmdargs = &a[1..a.len()];
    log::debug!("Command to be spawned is {}", &cmdname);
    log::debug!("Arguments are {:?}", &cmdargs);
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    let mut cmd = std::process::Command::new(cmdname)
        .args(cmdargs)
        .spawn()
        .expect("failed to execute application that to be measured");
    match cmd.try_wait() {
        Ok(None) => {
            let mut sys = System::new();
            let pid = &(cmd.id() as i32);
            log::debug!("Spawned command PID is: {}", &pid);
            let sec = time::Duration::from_secs(1);
            loop {
                match cmd.try_wait() {
                    Ok(None) => {
                        sys.refresh_all();
                        let processes = &sys.processes();
                        if processes.contains_key(&Pid::from(*pid)) {
                            nrevt::process_process_event(&c, &e, &pid, &Vec::new());
                            thread::sleep(sec);
                        } else {
                            log::debug!("Spawned command PID is: {} no more", &pid);
                            break;
                        }
                    }
                    Ok(Some(_)) => {
                        break
                    }
                    Err(err) => {
                        log::error!("Error happens while trying tu measure process: {err}");
                        std::process::exit(14);
                    }
                }
            }
            let stop = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
            nrtrace::process_trace_with_timestamp(&c, &ts, &s, &t, &i, &p, cmdname, &(stop-start), &Vec::new());
        }
        Ok(Some(_)) => {
            log::error!("Child process return OK, but we do not know what to do");
            std::process::exit(14);
        }
        Err(err) => {
            log::error!("Error happens while trying to measure process: {err}");
            std::process::exit(14);
        }
    }
}
