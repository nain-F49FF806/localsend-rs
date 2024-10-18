use super::common_fields::{DeviceInfo, IsAnnounce, Port, PreferDownload, Protocol};

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
    device_info: DeviceInfo,
    port: Port,
    protocol: Protocol,
    download: Option<PreferDownload>,
    announce: IsAnnounce,
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
struct MulticastResponse {
    device_info: DeviceInfo,
    port: Port,
    protocol: Protocol,
    download: Option<PreferDownload>,
    announce: Option<IsAnnounce>,
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
struct LegacyRegister {
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

struct LegacyRegisterResponse {
    device_info: DeviceInfo,
    download: Option<PreferDownload>,
}
