use std::{net::Ipv4Addr, path::PathBuf};

use argh::FromArgs;
use derive_getters::Getters;

use crate::utils::dbgr;
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
    Download(DownloadArgs),
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

/// download files from some localsend device
#[derive(FromArgs, PartialEq, Debug, Getters)]
#[argh(subcommand, name = "download")]
pub struct DownloadArgs {
    /// ip address of sender
    #[argh(positional)]
    sender: Ipv4Addr,
    /// specify port, if not using default (53317)
    #[argh(option, default = "53317")]
    port: u16,
    /// security pin if set
    #[argh(option)]
    pin: Option<String>,
    /// where to download the files
    #[argh(positional, default = "get_current_dir()")]
    destination: PathBuf,
}

fn get_current_dir() -> PathBuf {
    std::env::current_dir().inspect_err(dbgr).unwrap()
}
