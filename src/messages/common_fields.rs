//! Fields common to most messages

/// Alias
///
/// A name to present to other devices.
/// Should be recognizable and easy to discern.
pub(super) struct Alias(String);

/// Localsend protocol version
pub(super) struct Version {
    major: u32,
    minor: u32,
}

/// Device Model
///
/// ex Samsung / Windows / Linux
pub(super) struct DeviceModel(String);

/// Device type:
///  mobile | desktop | web | headless | server
pub(super) enum DeviceType {
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
pub(super) struct Fingerprint(String);

/// Device Info
pub(super) struct DeviceInfo {
    alias: Alias,
    version: Version,
    device_model: Option<DeviceModel>,
    device_type: DeviceType,
    fingerprint: Fingerprint,
}

/// Port
pub(super) struct Port(u16);

/// Protocol:
/// http / https
pub(super) enum Protocol {
    Http,
    Https,
}
