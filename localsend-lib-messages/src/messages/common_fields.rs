mod common_device_fields;
mod common_file_fields;

pub(super) use common_device_fields::*;
pub(super) use common_file_fields::*;
use derive_more::derive::{Constructor, From};
use serde::{Deserialize, Serialize};
/// SHA256
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
// pub(super) struct SHA256([u8; 32]);
pub(super) struct SHA256(String);
