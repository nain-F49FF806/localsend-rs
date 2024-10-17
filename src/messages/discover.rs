use super::common_fields::{Alias, DeviceModel, DeviceType, Fingerprint, Port, Protocol, Version};

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
struct MulticastAnnounce {
    alias: Alias,
    version: Version,
    device_model: Option<DeviceModel>,
    device_type: DeviceType,
    fingerprint: Fingerprint,
    port: Port,
    protocol: Protocol,
    download: Option<bool>,
    announce: bool,
}

/// Response
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
struct MulticastResponse {
    alias: Alias,
    version: Version,
    device_model: DeviceModel,
    device_type: DeviceType,
    fingerprint: Fingerprint,
    port: Port,
    protocol: Protocol,
    download: Option<bool>,
    announce: Option<bool>,
}
