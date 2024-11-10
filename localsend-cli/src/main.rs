use localsend_lib_types::messages;
use localsend_lib_types::messages::common_fields::{DeviceInfo, DeviceType, Protocol};
use localsend_lib_types::messages::discover::{MulticastAnnounce, MulticastResponse};
use multicast_socket::MulticastSocket;
use reqwest::Url;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use std::{net::SocketAddrV4, thread};

fn main() {
    println!("Hello, world!");
    let mulicast_address = SocketAddrV4::new([224, 0, 0, 167].into(), 53317);
    let socket = MulticastSocket::all_interfaces(mulicast_address).unwrap();

    thread::scope(|scope| {
        let (cadet1_tx, cadet2_rx) = channel();
        let (cadet2_tx, cadet1_rx) = channel::<MulticastResponse>();
        // multicast cadet
        scope.spawn(move || loop {
            if let Ok(udp_message) = socket.receive() {
                dbg!(&udp_message.origin_address);
                let data_string = String::from_utf8(udp_message.data).unwrap();
                dbg!(&data_string);
                if let Ok(announce) = serde_json::from_str::<MulticastAnnounce>(&data_string) {
                    dbg!(&announce);
                    if let Ok(()) = cadet1_tx.send((udp_message.origin_address, announce)) {
                        if let Ok(announce_response) =
                            cadet1_rx.recv_timeout(Duration::from_millis(250))
                        {
                            let _ =
                                socket.broadcast(&serde_json::to_vec(&announce_response).unwrap());
                        };
                    };
                };
            };
        });

        let req = reqwest::blocking::Client::new();
        // http time
        scope.spawn(move || loop {
            if let Ok((address, announce)) = cadet2_rx.recv() {
                let ip = address.ip();
                let port = announce.port().dissolve();
                let url = format!("http://{}:{}", ip, port);
                dbg!(&url);
                let self_announce_response = MulticastResponse::new(
                    DeviceInfo::new(
                        "Anonimoan Monari".to_string().into(),
                        "2.1".to_string().into(),
                        Some("localsend.rs-cli".to_string().into()),
                        DeviceType::Headless,
                        "randomfingerprint0".to_string().into(),
                    ),
                    53317.into(),
                    Protocol::Http,
                    Some(true.into()),
                    Some(serde_bool::False),
                );
                let self_announce_res_str = serde_json::to_string(&self_announce_response).unwrap();
                dbg!(&self_announce_res_str);
                let res = req.post(url).body(self_announce_res_str).build();
                if let Err(err) = res {
                    dbg!(err);
                };
                let _ = cadet2_tx.send(self_announce_response);
            };
        });

        //
    });
}
