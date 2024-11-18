use std::net::Ipv4Addr;

pub const MULTICAST_IP: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 167);
pub const LOCALSEND_PORT: u16 = 53317;

// pub const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
pub const FOX_USER_AGENT: &str = concat!(
    "Mozilla/5.0 (X11; Linux;) Gecko/20100101 ",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);
