use std::{net::SocketAddrV4, sync::mpsc, thread, time::Duration};

use localsend_lib_types::messages::discover::MulticastAnnounce;
use multicast_socket::MulticastSocket;

use crate::args::DiscoverArgs;

pub fn discover(discover_args: DiscoverArgs) {
    let _listen_broadcasts_handle = thread::spawn(listen_broadcasts);
    thread::sleep(Duration::from_secs(discover_args.timeout()));
}

fn listen_broadcasts() {
    println!("Listening for broadcasts!");
    let mulicast_address = SocketAddrV4::new([224, 0, 0, 167].into(), 53317);
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
    println!("Announcing ourselves over multicast");
    todo!()
}
