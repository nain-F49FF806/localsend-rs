//! Fields common to most messages

use derive_more::derive::{Constructor, From};
// use derive_new::new;
use serde::{Deserialize, Serialize};

/// Alias
///
/// A name to present to other devices.
/// Should be recognizable and easy to discern.
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
pub(in super::super) struct Alias(String);

/// Localsend protocol version (major.minor)
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
pub(in super::super) struct Version(String);

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
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
pub(in super::super) struct DeviceModel(String);

/// Device type:
///  mobile | desktop | web | headless | server
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(in super::super) enum DeviceType {
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
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
pub(in super::super) struct Fingerprint(String);

/// Device Info
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(in super::super) struct DeviceInfo {
    alias: Alias,
    version: Version,
    device_model: Option<DeviceModel>,
    device_type: DeviceType,
    fingerprint: Fingerprint,
}

/// Port
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
pub(in super::super) struct Port(u16);

/// Protocol:
/// http / https
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(in super::super) enum Protocol {
    Http,
    Https,
}

/// Download
///
/// Prefer download API (recievers pull) over upload API (senders push)
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
pub(in super::super) struct PreferDownload(bool);

// /// Announce
// ///
// /// Denotes if a discovery message is of type announce or response.
// /// A discovery response is only triggered when announce is true.
// #[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
// pub(in super::super) struct IsAnnounce(bool);

/// Session ID
///
/// A shared secret that can be used to authorise upload / download,to / from server
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
pub struct SessionId(String);
