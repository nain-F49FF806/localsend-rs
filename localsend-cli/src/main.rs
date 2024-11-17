mod args;
mod constants;
mod discover;
mod download;
mod state;
mod utils;

use args::{Args, SubCommand};
use discover::discover;
use download::download;

fn main() {
    let args: Args = argh::from_env();
    // println!("Args {:?}", args);
    match args.subcommand {
        SubCommand::Discover(discover_args) => discover(discover_args),
        SubCommand::Download(download_args) => download(download_args),
    }
}
