use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddrV4},
    thread,
    time::Duration,
};

use derive_more::derive::Display;
use localsend_lib_types::messages::{
    common_fields::{DeviceInfo, Fingerprint, Port, Protocol},
    discover::{MulticastAnnounce, MulticastCommon, MulticastMessage},
};
use multicast_socket::{Interface, MulticastSocket};

use crate::{
    args::DiscoverArgs,
    constants::{LOCALSEND_PORT, MULTICAST_IP},
    state::load_state,
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

fn listen_broadcasts(device_info: DeviceInfo) -> PeersMap {
    println!("Listening for broadcasts!");
    let mulicast_address = SocketAddrV4::new(MULTICAST_IP, LOCALSEND_PORT);
    let socket = MulticastSocket::all_interfaces(mulicast_address).unwrap();
    let mut peers: PeersMap = HashMap::new();
    loop {
        let Ok(udp_message) = socket.receive() else {
            continue;
        };
        let peer_address = udp_message.origin_address.ip();
        let message_string =
            String::from_utf8(udp_message.data).expect("Message should be valid utf8 string");
        let Ok(multicast_message) = serde_json::from_str(&message_string) else {
            dbg!(peer_address, message_string);
            continue;
        };
        let multicast_common: MulticastCommon = match multicast_message {
            MulticastMessage::Announce(announce) => announce.multicast_common().clone(),
            MulticastMessage::Response(response) => response.multicast_common().clone(),
        };
        let peer_fingerprint = multicast_common.device_info().fingerprint();
        if peer_fingerprint != device_info.fingerprint() {
            let peer_download_mode = if let Some(prefer_download_mode) = multicast_common.download()
            {
                *prefer_download_mode
            } else {
                false
            };
            let peer_info = PeerInfo {
                device_info: multicast_common.device_info().clone(),
                address: *peer_address,
                port: *multicast_common.port(),
                protocol: *multicast_common.protocol(),
                download_mode: peer_download_mode,
            };
            if !peers.contains_key(peer_fingerprint) {
                println!("New peer: {}", &peer_info);
            } else {
                // println!("Updaing peer {:?}", peer_fingerprint);
            }
            peers.insert(peer_fingerprint.clone(), peer_info);
        };
    }
}

fn announce_broadcast(device_info: DeviceInfo, interval: u64) {
    println!("Announcing ourselves over multicast");
    let mulicast_address = SocketAddrV4::new(MULTICAST_IP, LOCALSEND_PORT);
    let socket = MulticastSocket::all_interfaces(mulicast_address).unwrap();

    let self_announce = MulticastAnnounce::from(MulticastCommon::new(
        device_info,
        LOCALSEND_PORT.into(),
        Protocol::Http,
        Some(true),
    ));
    let announce_string = serde_json::to_string(&self_announce).expect("fix this serialization");
    let announce_bytes = announce_string.as_bytes();
    loop {
        let result = socket.send(announce_bytes, &Interface::Default);
        if let Err(e) = result {
            dbg!(e);
        }
        thread::sleep(Duration::from_secs(interval));
    }
}

type PeersMap = HashMap<Fingerprint, PeerInfo>;

#[derive(Display, Debug)]
#[display("{}", self.terminal_display())]

struct PeerInfo {
    pub device_info: DeviceInfo,
    pub address: Ipv4Addr,
    pub port: Port,
    pub protocol: Protocol,
    pub download_mode: bool,
}

impl PeerInfo {
    fn terminal_display(&self) -> String {
        let full_address = format!("{}:{}", self.address, self.port);
        let url = format!(
            "{}://{}",
            self.protocol.to_string().to_lowercase(),
            full_address
        );
        let link = terminal_link::Link::new(&full_address, &url);
        format!(
            "{} @{} {}",
            self.device_info,
            link,
            if self.download_mode { "📥" } else { "📤" }
        )
    }
}
