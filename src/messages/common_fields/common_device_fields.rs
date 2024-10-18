//! Fields common to most messages

/// Alias
///
/// A name to present to other devices.
/// Should be recognizable and easy to discern.
pub(in super::super) struct Alias(String);

/// Localsend protocol version
pub(in super::super) struct Version {
    major: u32,
    minor: u32,
}

/// Device Model
///
/// ex Samsung / Windows / Linux
pub(in super::super) struct DeviceModel(String);

/// Device type:
///  mobile | desktop | web | headless | server
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
pub(in super::super) struct Fingerprint(String);

/// Device Info
pub(in super::super) struct DeviceInfo {
    alias: Alias,
    version: Version,
    device_model: Option<DeviceModel>,
    device_type: DeviceType,
    fingerprint: Fingerprint,
}

/// Port
pub(in super::super) struct Port(u16);

/// Protocol:
/// http / https
pub(in super::super) enum Protocol {
    Http,
    Https,
}

/// Download
///
/// Prefer download API (recievers pull) over upload API (senders push)
pub(in super::super) struct PreferDownload(bool);

/// Announce
///
/// Denotes if a discovery message is of type announce or response.
/// A discovery response is only triggered when announce is true.
pub(in super::super) struct IsAnnounce(bool);

/// Session ID
///
/// A shared secret that can be used to authorise upload / download,to / from server
pub(in super::super) struct SessionId(String);
