use derive_getters::Getters;
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common_fields::{DeviceInfo, Port, PreferDownload, Protocol, Version};

/// Common fields for Multicast Announce / Multicast Response
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq, Getters, Clone)]
pub struct MulticastCommon {
    #[serde(flatten)]
    device_info: DeviceInfo,
    port: Port,
    protocol: Protocol,
    download: Option<PreferDownload>,
}
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
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq, Getters)]
pub struct MulticastAnnounce {
    version: Version,
    #[serde(flatten)]
    multicast_common: MulticastCommon,
    announce: serde_bool::True,
}

impl From<MulticastCommon> for MulticastAnnounce {
    fn from(value: MulticastCommon) -> Self {
        MulticastAnnounce {
            version: Version::default(),
            multicast_common: value,
            announce: serde_bool::True,
        }
    }
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
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq, Getters)]
pub struct MulticastResponse {
    version: Version,
    #[serde(flatten)]
    multicast_common: MulticastCommon,
    announce: Option<serde_bool::False>,
}

impl From<MulticastCommon> for MulticastResponse {
    fn from(value: MulticastCommon) -> Self {
        MulticastResponse {
            version: Version::default(),
            multicast_common: value,
            announce: Some(serde_bool::False),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MulticastMessage {
    Announce(MulticastAnnounce),
    Response(MulticastResponse),
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
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq, Getters)]
pub struct LegacyRegister {
    version: Version,
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
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq, Getters)]
pub struct LegacyRegisterResponse {
    version: Version,
    #[serde(flatten)]
    device_info: DeviceInfo,
    download: Option<PreferDownload>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::messages::{
        common_fields::{
            Alias, DeviceInfo, DeviceModel, DeviceType, Fingerprint, Port, PreferDownload,
            Protocol, Version,
        },
        discover::MulticastCommon,
    };

    use super::{LegacyRegister, LegacyRegisterResponse, MulticastAnnounce, MulticastResponse};

    #[test]
    fn multicast_announce_deserialize_serialize() {
        let multicast_announce_json = json!(
            {
                "alias": "Nice Orange",
                "version": "2.0", // protocol version (major.minor)
                "deviceModel": "Samsung", // nullable
                "deviceType": "mobile", // mobile | desktop | web | headless | server, nullable
                "fingerprint": "random string",
                "port": 53317,
                "protocol": "https", // http | https
                "download": true, // if the download API (5.2 and 5.3) is active (optional, default: false)
                "announce": true
            }
        );

        let device_info = DeviceInfo::new(
            Alias::new("Nice Orange".into()),
            Some(DeviceModel::new("Samsung".into())),
            DeviceType::Mobile,
            Fingerprint::new("random string".into()),
        );
        let constructed_multicast_announce = MulticastAnnounce::new(
            Version::new("2.0".into()),
            MulticastCommon::new(
                device_info,
                Port::new(53317),
                Protocol::Https,
                Some(PreferDownload::new(true)),
            ),
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

    #[test]
    fn multicast_response_deserialize_serialize() {
        let response_json_1 = json!(
            {
                "alias": "Secret Banana",
                "version": "2.0",
                "deviceModel": "Windows",
                "deviceType": "desktop",
                "fingerprint": "random string", // ignored in HTTPS mode
                "port": 53317,
                "protocol": "https",
                "download": true, // if the download API (5.2 and 5.3) is active (optional, default: false)
            }
        );
        let response_json_2 = json!(
            {
                "alias": "Secret Banana",
                "version": "2.0",
                "deviceModel": "Windows",
                "deviceType": "desktop",
                "fingerprint": "random string",
                "port": 53317,
                "protocol": "https",
                "download": true,
                "announce": false,
            }
        );
        let constructed_response_1 = MulticastResponse::new(
            "2.0".to_owned().into(),
            MulticastCommon::new(
                DeviceInfo::new(
                    "Secret Banana".to_owned().into(),
                    Some("Windows".to_owned().into()),
                    DeviceType::Desktop,
                    "random string".to_owned().into(),
                ),
                53317.into(),
                Protocol::Https,
                Some(PreferDownload::new(true)),
            ),
            None,
        );
        let constructed_response_2 = MulticastResponse::new(
            "2.0".to_owned().into(),
            MulticastCommon::new(
                DeviceInfo::new(
                    "Secret Banana".to_owned().into(),
                    Some("Windows".to_owned().into()),
                    DeviceType::Desktop,
                    "random string".to_owned().into(),
                ),
                53317.into(),
                Protocol::Https,
                Some(PreferDownload::new(true)),
            ),
            Some(serde_bool::False),
        );
        // Deserialize
        let read_response_1: MulticastResponse =
            serde_json::from_value(response_json_1.clone()).unwrap();
        assert_eq!(constructed_response_1, read_response_1);
        let read_response_2: MulticastResponse =
            serde_json::from_value(response_json_2.clone()).unwrap();
        assert_eq!(constructed_response_2, read_response_2);
        // Serialize
        let written_response_json_1 = serde_json::to_value(constructed_response_1).unwrap();
        assert_eq!(response_json_1, written_response_json_1);
        let written_response_json_2 = serde_json::to_value(constructed_response_2).unwrap();
        assert_eq!(response_json_2, written_response_json_2);
    }

    #[test]
    fn legacy_register_deserialize_serialize() {
        let request_json = json!(
            {
                "alias": "Secret Banana",
                "version": "2.0", // protocol version (major.minor)
                "deviceModel": "Windows",
                "deviceType": "desktop",
                "fingerprint": "random string", // ignored in HTTPS mode
                "port": 53317,
                "protocol": "https", // http | https
                "download": true, // if the download API (5.2 and 5.3) is active (optional, default: false)
            }
        );
        let constructed_request = LegacyRegister::new(
            Version::new("2.0".into()),
            DeviceInfo::new(
                Alias::new("Secret Banana".into()),
                Some(DeviceModel::new("Windows".into())),
                DeviceType::Desktop,
                Fingerprint::new("random string".into()),
            ),
            Port::new(53317),
            Protocol::Https,
            Some(PreferDownload::new(true)),
        );

        // Deserialize
        let read_request: LegacyRegister = serde_json::from_value(request_json.clone()).unwrap();
        assert_eq!(constructed_request, read_request);
        // Serialize
        let written_multicast_announce_json = serde_json::to_value(constructed_request).unwrap();
        assert_eq!(request_json, written_multicast_announce_json);
    }

    #[test]
    fn legacy_response_deserialize_serialize() {
        let response_json = json!(
            {
              "alias": "Nice Orange",
              "version": "2.0",
              "deviceModel": "Samsung",
              "deviceType": "mobile",
              "fingerprint": "random string", // ignored in HTTPS mode
              "download": true, // if the download API (5.2 and 5.3) is active (optional, default: false)
            }
        );
        let constructed_response = LegacyRegisterResponse::new(
            Version::new("2.0".into()),
            DeviceInfo::new(
                Alias::new("Nice Orange".into()),
                Some("Samsung".to_owned().into()),
                DeviceType::Mobile,
                Fingerprint::new("random string".into()),
            ),
            Some(PreferDownload::new(true)),
        );

        // Deserialize
        let read_response: LegacyRegisterResponse =
            serde_json::from_value(response_json.clone()).unwrap();
        assert_eq!(constructed_response, read_response);
        debug_assert_eq!(constructed_response, read_response);
        // Serialize
        let written_multicast_announce_json = serde_json::to_value(constructed_response).unwrap();
        assert_eq!(response_json, written_multicast_announce_json);
    }
}
