use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddrV4},
    thread,
    time::Duration,
};

use localsend_lib_types::messages::{
    common_fields::{DeviceInfo, Fingerprint, Port, Protocol, Version},
    discover::{MulticastAnnounce, MulticastResponse},
};
use multicast_socket::{Interface, MulticastSocket};

use crate::{
    args::DiscoverArgs,
    config::load_state,
    constants::{LOCALSEND_PORT, MULTICAST_IP},
};

/// Discover nearby localsend devices/peers
/// Currently support only Multicast Announce and Multicast Response
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
    let mut peers: HashMap<Fingerprint, PeerInfo> = HashMap::new();
    loop {
        let Ok(udp_message) = socket.receive() else {
            continue;
        };
        let peer_address = udp_message.origin_address.ip();
        dbg!(peer_address);
        let message_string = String::from_utf8(udp_message.data).unwrap();

        if let Ok(announce) = serde_json::from_str::<MulticastAnnounce>(&message_string) {
            let peer_fingerprint = announce.device_info().fingerprint();
            if peer_fingerprint != device_info.fingerprint() {
                let peer_download_mode = if let Some(prefer_download_mode) = announce.download() {
                    prefer_download_mode.clone().dissolve()
                } else {
                    false
                };
                let peer_info = PeerInfo {
                    device_info: announce.device_info().clone(),
                    address: *peer_address,
                    port: *announce.port(),
                    download_mode: peer_download_mode,
                };
                if !peers.contains_key(peer_fingerprint) {
                    println!("Adding new peer {:?}", peer_fingerprint);
                    dbg!(&peer_info);
                } else {
                    // println!("Updaing peer {:?}", peer_fingerprint);
                }
                peers.insert(peer_fingerprint.clone(), peer_info);
            };
        };

        if let Ok(response) = serde_json::from_str::<MulticastResponse>(&message_string) {
            let peer_fingerprint = response.device_info().fingerprint();
            if peer_fingerprint != device_info.fingerprint() {
                let peer_download_mode = if let Some(prefer_download_mode) = response.download() {
                    prefer_download_mode.clone().dissolve()
                } else {
                    false
                };
                let peer_info = PeerInfo {
                    device_info: response.device_info().clone(),
                    address: *peer_address,
                    port: *response.port(),
                    download_mode: peer_download_mode,
                };
                if !peers.contains_key(peer_fingerprint) {
                    println!("Adding new peer {:?}", peer_fingerprint);
                    dbg!(&peer_info);
                } else {
                    // println!("Updaing peer {:?}", peer_fingerprint);
                }
                peers.insert(peer_fingerprint.clone(), peer_info);
            }
        };
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

#[derive(Debug)]
struct PeerInfo {
    pub device_info: DeviceInfo,
    pub address: Ipv4Addr,
    pub port: Port,
    pub download_mode: bool,
}
