# nrapm - New Relic instrumentation for Unix (and not only) shells.

`nrapm` is a shell command that sends metric events and logs to the [New Relic monitoring and observability platform](https://www.newrelic.com). In fact, this tool is your [APM instrument](https://newrelic.com/platform/application-monitoring) for shell scripts. `nrapm` is a very efficient and lean application on most platforms, wuth a binary size of about 4Mb, and can be used in most embedded applications as well.

## Compilation and Installation

`nrapm` is written in [Rust](https://www.rust-lang.org/) and requires a minimum Rust version 1.62 to [compile from source](https://www.rust-lang.org/tools/install). There are no other dependencies.

Verify that your build environment is ready, by running:

```shells
$ cargo --version
cargo 1.62.0 (a748cf5a3 2022-06-08)
```

Now, you can check out the [nrapm source code](https://github.com/vulogov/nrapm) to your build host, change directory to the project root.

To build development version of `nrapm`, you can run: 

```shells
$ cargo build
   Compiling nrapm v1.0.0 (........../nrapm)
    Finished dev [unoptimized + debuginfo] target(s) in 2.98s
```

 To build an optimized `nrapm` binary for your target CPU and platform, you can run:

```shells
$ cargo rustc --release -- -C target-cpu=native
   Compiling nrapm v1.0.0 (............/nrapm)
    Finished release [optimized] target(s) in 4.92s
```

 And now, you can install `nrapm` to your desired location. 

## Running nrapm

`nrapm` provides relevant help messages.

```shells
nrapm
[2022-08-07T14:28:27Z TRACE nrapm] nrapm main() function is reached
nrapm 1.0
Vladimir Ulogov <vulogov@newrelic.com>
CLI interface to a New Relic

USAGE:
    nrapm [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -e
    -h, --help                       Print help information
        --hostname <HOSTNAME>        [default: ........]
        --nr-account <NR_ACCOUNT>    [default: ]
        --nr-api <NR_API>            [default: ]
        --nr-event <NR_EVENT>        [default: insights-collector.newrelic.com]
        --nr-insert <NR_INSERT>      [default: ]
        --nr-log <NR_LOG>            [default: log-api.newrelic.com]
        --nr-metric <NR_METRIC>      [default: metric-api.newrelic.com]
        --nr-trace <NR_TRACE>        [default: trace-api.newrelic.com]
        --timestamp <TIMESTAMP>      [default: 1659882507]
    -V, --version                    Print version information

SUBCOMMANDS:
    eval      Evaluate expressions
    event     Send Event to a New Relic
    help      Print this message or the help of the given subcommand(s)
    log       Send Logs to a New Relic
    metric    Send Metric to a New Relic
    trace     Send Trace data to a New Relic
```

In general, `nrapm` is called with a subcommand:

- `event` - to send New Relic events
- `log` - to send New Relic log messages
- `metric` - to send New Relic metrics

Each of those commands has its own individual set of keys, which we will discuss below. Before that, let's set up a New Relic environment.

### Setting up New Relic environment

To successfully run `nrapm`, you will need three artifacts from New Relic

- *Account number* - The numeric account ID associated with your [New Relic account](https://one.newrelic.com)
- *New Relic API key* - You can generate/view your API key in the "New Relic API Keys" section under "Preferences". API keys generally start with the letters "NRAK"
- *New Relic Ingest key* - You can also generate/view your INGEST key in the "New Relic API Keys" section under "Preferences". INGEST keys generally end with the letters "NRAL"

There are two ways to pass this info to `nrapm`.

#### Environment Variables

This is the recommended method. You need to set three environment variables:

- *NEWRELIC_ACCOUNT* - set to your Account number
- *NEWRELIC_API* - set to your New Relic API key
- *NEWRELIC_INSERTKEY* - set to your New Relic INGEST key.

#### Parameters to the `nrapm` Command Line

- `--nr-account` - set to your Account number
- `--nr-api` - set to your New Relic API key
- `--nr-insert` - set to your New Relic INGEST key.

This is an insecure method of passing critical information to `nrapm` and we DO NOT RECOMMEND this, as the command line parameters will be viewable to any user on the system who can the `ps` command.

### Curb the debug output

By default, `nrapm` outputs with debug level "ERROR", so it only displays error messages. You can set environment variable NRAPM_LOG_LEVEL to your desired level of output from `nrapm`. Available options are:

- `trace` - lots of output and that is default.
- `debug` - much more compact output
- `warning` - very few messages
- `error` - report only errors. Set this for production.

You can also control the output with command line paraeters `-d` or `--debug` to the `nrapm` command. You can pass the flag multiple times to increase the verbosity.

### Passing positional parameters to `nrapm`

A lot of information that is part of the event/metric/log can passed via positional parameters -- separated from regular parameters of the command by double-dash. For events and metrics, positional parameters are in key-value form, where key separated from value with "equal" sign. For example:

```
-- answer=42 greetings="Hello world"
```

For the New Relic log tool, each positional parameter is a string, and each string is a separate log message. For example:

```
-- "First log message" "Second log message"
```

### Sending event

Sub-Command `event` for sending event have a format:

```
nrapm event --evt-type "EventType" -- positional parameters
```

Where CLI parameter "-e" or "--evt-type" defines event type. Default event type is "ShellEvent". Positional parameters are key-value pairs passed to event. Example:

```shells
nrapm event -e MyEvent -- answer=42 greeting="Hello world"
```
![sent event](documentation/event.png)

### Sending metrics

Sub-Command `metric` for sending metrics to New Relic observability platform, which accepts the following parameters:

- "-m" or "--metric-type" this defines a type of the metric, with default is "gauge". Other metric types could be referenced from [here](https://docs.newrelic.com/docs/data-apis/understand-data/metric-data/metric-data-type/)
- "-n" or "--name" metric name
- "-v" or "--value" metric value. nrapm makes a best effort to automatically detect numeric/boolean/string value type. You can pass positional parameters in a orm of "key=value" to add for your metric. Example:

```
nrapm metric --name "my.application.metric" --value 42 -- pi=3.14 greetings="Hello world"
```
![sent metric](documentation/metric.png)

### Sending logs

Sub-command `log` can be used for sending log messages to the New Relic observability platform, via the following parameters:

- "-l" or "--log-type" - string that defines a log type. Default is a "syslog"
- "-s" or "--service" - string that defines a service that sends this log message. Default is "shell"

Positional parameters are the strings that will be passed as log messages. Example:

```
nrapm log  -- "Log line 1" "Log line 2"
```
![sent log](documentation/log.png)

## Conclusion

`nrapm` is a simple tool that allows you to instrument your shell scripts without using any complicated APM tool going into excessive development. We appreciate your feedback, critiques, [bug reports](https://github.com/vulogov/nrapm/issues), and [pull requests](https://github.com/vulogov/nrapm/pulls).
