# Crazyflie CLI

This is a command line interface (CLI) for the Crazyflie 2.X written in Rust. It's intended to be used
during development to easily log the console, set parameters and get logging variables. It's not
intended to be used from any scripts to make the Crazyflie do things, then using the
[Crazyflie python library](https://github.com/bitcraze/crazyflie-lib-python) is preferred.

## Installation

If you would like to install the CLI, you can do so by running the following command:

```text
cd cf-cli
cargo install
```

Now the `cf-cli` command should be available in your terminal.

Or if you would like to run it from source use the following command:

```text
cargo run -- <args>
```

For example:

```text
cargo run -- select
```

## Usage

To see how to use the CLI type ```cf-cli``` in your terminal and you will get the following help message:

```text
Crazyflie CLI

Usage: cf-cli [OPTIONS] <COMMAND>

Commands:
  log      Access to the log subsystem
  param    Access to the parameter subsystem
  scan     List the Crazyflies found while scanning (on the selected address)
  select   Scan for Crazyflies and select which one to save for later interactions
  console  Print the console text from a Crazyflie
  help     Print this message or the help of the given subcommand(s)

Options:
  -a, --address <ADDRESS>  Specify address [default: E7E7E7E7E7]
  -h, --help               Print help
  -V, --version            Print version
```

To use the CLI you must first select which URI to use, this is done by scanning for available Crazyflies
and selecting the one you prefer.

```text
cf-cli select
```

Now this URI will be used in all commands until a new one is selected. For instance a parameter
can be set using the following command:

```text
cf-cli param set motorPowerSet.enable 1
```

And a log variable can be printed using the following command:

```text
cf-cli log print stabilizer.roll 100

```
