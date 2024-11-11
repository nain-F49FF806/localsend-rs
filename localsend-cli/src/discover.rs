use std::{net::SocketAddrV4, thread, time::Duration};

use localsend_lib_types::messages::{
    common_fields::{Protocol, Version},
    discover::MulticastAnnounce,
};
use multicast_socket::{Interface, MulticastSocket};

use crate::{
    args::DiscoverArgs,
    config::load_state,
    constants::{LOCALSEND_PORT, MULTICAST_IP},
};

pub fn discover(discover_args: DiscoverArgs) {
    let _announce_broadcast_handle = thread::spawn(announce_broadcast);
    let _listen_broadcasts_handle = thread::spawn(listen_broadcasts);
    thread::sleep(Duration::from_secs(discover_args.timeout()));
}

fn listen_broadcasts() {
    println!("Listening for broadcasts!");
    let mulicast_address = SocketAddrV4::new(MULTICAST_IP, LOCALSEND_PORT);
    let socket = MulticastSocket::all_interfaces(mulicast_address).unwrap();
    loop {
        if let Ok(udp_message) = socket.receive() {
            dbg!(&udp_message.origin_address);
            let data_string = String::from_utf8(udp_message.data).unwrap();
            if let Ok(announce) = serde_json::from_str::<MulticastAnnounce>(&data_string) {
                dbg!(&announce);
            };
        };
    }
}

fn announce_broadcast() {
    let state = load_state();
    let device_info = state.device_info;
    println!("Announcing ourselves over multicast");
    let mulicast_address = SocketAddrV4::new(MULTICAST_IP, LOCALSEND_PORT);
    let socket = MulticastSocket::all_interfaces(mulicast_address).unwrap();

    let self_announce = MulticastAnnounce::new(
        Version::default(),
        device_info,
        LOCALSEND_PORT.into(),
        Protocol::Http,
        Some(true.into()),
        serde_bool::True,
    );
    let data_string = serde_json::to_string(&self_announce).expect("fix this serialization");
    match socket.send(data_string.as_bytes(), &Interface::Default) {
        Ok(bytes_sent) => {
            dbg!(bytes_sent);
        }
        Err(e) => {
            dbg!(e);
        }
    };
}
