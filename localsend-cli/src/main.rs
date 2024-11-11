mod args;
mod config;
mod constants;
mod discover;

use args::{Args, SubCommand};
use discover::discover;

fn main() {
    let args: Args = argh::from_env();
    println!("Args {:?}", args);
    match args.subcommand {
        SubCommand::Discover(discover_args) => discover(discover_args),
    }
}
