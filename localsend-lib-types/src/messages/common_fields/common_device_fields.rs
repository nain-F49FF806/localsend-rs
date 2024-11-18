//! Fields common to most messages

use derive_getters::{Dissolve, Getters};
use derive_more::derive::{AsRef, Constructor, Display, From};
// use derive_new::new;
use serde::{Deserialize, Serialize};

/// Alias
///
/// A name to present to other devices.
/// Should be recognizable and easy to discern.
#[derive(Display, Debug, Serialize, Deserialize, Constructor, From, PartialEq, Clone)]
#[from(forward)]
pub struct Alias(String);

/// Localsend protocol version (major.minor)
#[derive(Display, Debug, Serialize, Deserialize, Constructor, From, PartialEq, Clone)]
#[from(forward)]
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
#[derive(Display, Debug, Serialize, Deserialize, Constructor, From, PartialEq, Clone)]
#[from(forward)]
pub struct DeviceModel(String);

/// Device type:
///  mobile | desktop | web | headless | server
#[derive(Display, Debug, Serialize, Deserialize, PartialEq, Clone)]
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
#[derive(Display, Debug, Serialize, Deserialize, Constructor, From, PartialEq, Eq, Hash, Clone)]
#[from(forward)]
pub struct Fingerprint(String);

/// Device Info
#[derive(Display, Debug, Serialize, Deserialize, Constructor, From, PartialEq, Getters, Clone)]
#[serde(rename_all = "camelCase")]
#[display("{alias} ({} {device_type})", device_model.as_ref().unwrap_or(&"Generic".into()))]
pub struct DeviceInfo {
    alias: Alias,
    device_model: Option<DeviceModel>,
    device_type: DeviceType,
    fingerprint: Fingerprint,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        DeviceInfo::new(
            "GenericTurnip".into(),
            Some("Rust".into()),
            DeviceType::Headless,
            "XXXXXXXXXXXXXXXX".into(),
        )
    }
}

/// Port
#[derive(
    Clone, Copy, Display, Debug, Serialize, Deserialize, Constructor, From, PartialEq, Dissolve,
)]
pub struct Port(u16);

impl Default for Port {
    fn default() -> Self {
        Port::new(53317)
    }
}

/// Protocol:
/// http / https
#[derive(Display, Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Http,
    Https,
}

// /// Announce
// ///
// /// Denotes if a discovery message is of type announce or response.
// /// A discovery response is only triggered when announce is true.
// #[derive(Display, Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
// pub struct IsAnnounce(bool);

/// Session ID
///
/// A shared secret that can be used to authorise upload / download,to / from server
#[derive(Display, Debug, Serialize, Deserialize, Constructor, From, PartialEq, Clone, AsRef)]
#[from(forward)]
pub struct SessionId(String);
