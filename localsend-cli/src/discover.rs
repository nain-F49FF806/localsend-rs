use std::{net::SocketAddrV4, thread, time::Duration};

use localsend_lib_types::messages::{
    common_fields::{DeviceInfo, Protocol, Version},
    discover::MulticastAnnounce,
};
use multicast_socket::{Interface, MulticastSocket};

use crate::{
    args::DiscoverArgs,
    config::load_state,
    constants::{LOCALSEND_PORT, MULTICAST_IP},
};

pub fn discover(discover_args: DiscoverArgs) {
    let state = load_state();
    let device_info = state.device_info;
    let announce_interval = discover_args.announce_interval();
    let _announce_broadcast_handle = thread::spawn({
        // Capture a clone https://stackoverflow.com/a/74817347
        let device_info = device_info.clone();
        move || announce_broadcast(device_info.clone(), announce_interval)
    });
    {
        // Similar to above. Shadow Clone in scope then capture.
        let device_info = device_info.clone();
        let _listen_broadcasts_handle =
            thread::spawn(move || listen_broadcasts(device_info.clone()));
    }

    thread::sleep(Duration::from_secs(discover_args.timeout()));
}

fn listen_broadcasts(device_info: DeviceInfo) {
    println!("Listening for broadcasts!");
    let mulicast_address = SocketAddrV4::new(MULTICAST_IP, LOCALSEND_PORT);
    let socket = MulticastSocket::all_interfaces(mulicast_address).unwrap();
    loop {
        let Ok(udp_message) = socket.receive() else {
            continue;
        };
        dbg!(&udp_message.origin_address);
        let data_string = String::from_utf8(udp_message.data).unwrap();
        let Ok(announce) = serde_json::from_str::<MulticastAnnounce>(&data_string) else {
            continue;
        };
        if announce.device_info().fingerprint() != device_info.fingerprint() {
            dbg!(&announce);
        }
    }
}

fn announce_broadcast(device_info: DeviceInfo, interval: u64) {
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
    let announce_string = serde_json::to_string(&self_announce).expect("fix this serialization");
    let announce_bytes = announce_string.as_bytes();
    loop {
        println!("Announcing ourselves over multicast");
        let result = socket.send(announce_bytes, &Interface::Default);
        if let Err(e) = result {
            dbg!(e);
        }
        thread::sleep(Duration::from_secs(interval));
    }
}
