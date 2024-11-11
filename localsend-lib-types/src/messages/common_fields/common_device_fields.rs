//! Fields common to most messages

use derive_getters::{Dissolve, Getters};
use derive_more::derive::{Constructor, From};
// use derive_new::new;
use serde::{Deserialize, Serialize};

/// Alias
///
/// A name to present to other devices.
/// Should be recognizable and easy to discern.
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq, Clone)]
pub struct Alias(String);

/// Localsend protocol version (major.minor)
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq, Clone)]
pub struct Version(String);

impl Default for Version {
    fn default() -> Self {
        Version::new("2.1".to_string())
    }
}
// impl FromStr for Version {
//     type Err = VersionDecodeError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         if let Some((major, minor)) = s.split_once('.') {
//             return Ok(Version {
//                 major: major.parse()?,
//                 minor: minor.parse()?,
//             });
//         } else {
//             Err(VersionDecodeError::NoDotFound)
//         }
//     }
// }

/// Device Model
///
/// ex Samsung / Windows / Linux
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq, Clone)]
pub struct DeviceModel(String);

/// Device type:
///  mobile | desktop | web | headless | server
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Mobile,
    Desktop,
    Web,
    Headless,
    Server,
}

/// Fingerprint
///
/// Unique string identifying the device.
/// Only used to ignore messages from self.
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq, Eq, Hash, Clone)]
pub struct Fingerprint(String);

/// Device Info
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    alias: Alias,
    device_model: Option<DeviceModel>,
    device_type: DeviceType,
    fingerprint: Fingerprint,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        DeviceInfo::new(
            "GenericTurnip".to_string().into(),
            Some("Rust".to_string().into()),
            DeviceType::Headless,
            "XXXXXXXXXXXXXXXX".to_string().into(),
        )
    }
}

/// Port
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Constructor, From, PartialEq, Dissolve)]
pub struct Port(u16);

impl Default for Port {
    fn default() -> Self {
        Port::new(53317)
    }
}

/// Protocol:
/// http / https
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Http,
    Https,
}

/// Download
///
/// Prefer download API (recievers pull) over upload API (senders push)
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq, Dissolve, Clone)]
pub struct PreferDownload(bool);

// /// Announce
// ///
// /// Denotes if a discovery message is of type announce or response.
// /// A discovery response is only triggered when announce is true.
// #[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
// pub struct IsAnnounce(bool);

/// Session ID
///
/// A shared secret that can be used to authorise upload / download,to / from server
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
pub struct SessionId(String);
