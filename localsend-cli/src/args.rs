use argh::FromArgs;
use derive_getters::Getters;
/// LocalSend cli
#[derive(FromArgs, PartialEq, Debug)]
pub struct Args {
    #[argh(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommand {
    Discover(DiscoverArgs),
}
/// discover surrounding localsend devices
#[derive(FromArgs, PartialEq, Debug, Getters)]
#[argh(subcommand, name = "discover")]
pub struct DiscoverArgs {
    /// do not announce or respond to announcements, just listen
    #[argh(switch)]
    silent: bool,
    /// how long to keep scanning, in seconds
    #[argh(option, default = "5")]
    timeout: u64,
    /// if not silent, how long to wait before repeating announcement,
    /// in seconds
    #[argh(option, default = "2")]
    announce_interval: u64,
}
