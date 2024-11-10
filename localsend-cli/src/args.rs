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
    /// how long to keep scanning, in seconds
    #[argh(option, default = "3")]
    timeout: u64,
    /// do not announce or respond to announcements, just listen
    #[argh(switch)]
    silent: bool,
}
