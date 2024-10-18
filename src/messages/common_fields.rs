mod common_device_fields;
mod common_file_fields;

pub(super) use common_device_fields::*;
pub(super) use common_file_fields::*;
/// SHA256
pub(super) struct SHA256([u8; 32]);
