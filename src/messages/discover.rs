use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

use super::common_fields::{DeviceInfo, Port, PreferDownload, Protocol};

/// Multicast UDP (Default) Announcement
///
/// At the start of the app, the following message will be sent to the multicast group:
///
/// ```json
/// {
///   "alias": "Nice Orange",
///   "version": "2.0", // protocol version (major.minor)
///   "deviceModel": "Samsung", // nullable
///   "deviceType": "mobile", // mobile | desktop | web | headless | server, nullable
///   "fingerprint": "random string",
///   "port": 53317,
///   "protocol": "https", // http | https
///   "download": true, // if the download API (5.2 and 5.3) is active (optional, default: false)
///   "announce": true
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
pub(crate) struct MulticastAnnounce {
    #[serde(flatten)]
    device_info: DeviceInfo,
    port: Port,
    protocol: Protocol,
    download: Option<PreferDownload>,
    announce: serde_bool::True,
}

/// MulticastAnnounce Response
///
/// Other LocalSend members notice the announce message and reply with their respective information.
///
/// First, an HTTP/TCP request is sent to the origin:
///
/// `POST /api/localsend/v2/register`
///
/// ```json
/// {
///   "alias": "Secret Banana",
///   "version": "2.0",
///   "deviceModel": "Windows",
///   "deviceType": "desktop",
///   "fingerprint": "random string", // ignored in HTTPS mode
///   "port": 53317,
///   "protocol": "https",
///   "download": true, // if the download API (5.2 and 5.3) is active (optional, default: false)
/// }
/// ```
/// As fallback, members can also respond with a Multicast/UDP message.
///
/// ```json
/// {
///     "alias": "Secret Banana",
///     "version": "2.0",
///     "deviceModel": "Windows",
///     "deviceType": "desktop",
///     "fingerprint": "random string",
///     "port": 53317,
///     "protocol": "https",
///     "download": true,
///     "announce": false,
///   }
/// ```
///
///   The fingerprint is only used to avoid self-discovering.
///
///   A response is only triggered when announce is true.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct MulticastResponse {
    #[serde(flatten)]
    device_info: DeviceInfo,
    port: Port,
    protocol: Protocol,
    download: Option<PreferDownload>,
    announce: Option<serde_bool::False>,
}

/// 3.2 HTTP (Legacy Mode)
///
/// This method should be used when multicast was unsuccessful.
///
/// Devices are discovered by sending this request to all local IP addresses.
///
/// `POST /api/localsend/v2/register`
///
/// Request
///
/// ```json
/// {
///   "alias": "Secret Banana",
///   "version": "2.0", // protocol version (major.minor)
///   "deviceModel": "Windows",
///   "deviceType": "desktop",
///   "fingerprint": "random string", // ignored in HTTPS mode
///   "port": 53317,
///   "protocol": "https", // http | https
///   "download": true, // if the download API (5.2 and 5.3) is active (optional, default: false)
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub(crate) struct LegacyRegister {
    #[serde(flatten)]
    device_info: DeviceInfo,
    port: Port,
    protocol: Protocol,
    download: Option<PreferDownload>,
}

/// Response
///
/// ```json
/// {
///   "alias": "Nice Orange",
///   "version": "2.0",
///   "deviceModel": "Samsung",
///   "deviceType": "mobile",
///   "fingerprint": "random string", // ignored in HTTPS mode
///   "download": true, // if the download API (5.2 and 5.3) is active (optional, default: false)
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub(crate) struct LegacyRegisterResponse {
    #[serde(flatten)]
    device_info: DeviceInfo,
    download: Option<PreferDownload>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::messages::common_fields::{Alias, DeviceModel, DeviceType, Fingerprint, Version};

    use super::*;

    #[test]
    fn multicast_announce_deserialize_serialize() {
        let multicast_announce_json = json!({
          "alias": "Nice Orange",
          "version": "2.0", // protocol version (major.minor)
          "deviceModel": "Samsung", // nullable
          "deviceType": "mobile", // mobile | desktop | web | headless | server, nullable
          "fingerprint": "random string",
          "port": 53317,
          "protocol": "https", // http | https
          "download": true, // if the download API (5.2 and 5.3) is active (optional, default: false)
          "announce": true
        });

        let device_info = DeviceInfo::new(
            Alias::new("Nice Orange".into()),
            Version::new("2.0".into()),
            Some(DeviceModel::new("Samsung".into())),
            DeviceType::Mobile,
            Fingerprint::new("random string".into()),
        );
        let constructed_multicast_announce = MulticastAnnounce::new(
            device_info,
            Port::new(53317),
            Protocol::Https,
            Some(PreferDownload::new(true)),
            serde_bool::True,
        );
        // Deserialize
        let read_multicast_announce: MulticastAnnounce =
            serde_json::from_value(multicast_announce_json.clone()).unwrap();
        assert_eq!(constructed_multicast_announce, read_multicast_announce);
        // Serialize
        let written_multicast_announce_json =
            serde_json::to_value(constructed_multicast_announce).unwrap();
        assert_eq!(multicast_announce_json, written_multicast_announce_json);
    }
}
