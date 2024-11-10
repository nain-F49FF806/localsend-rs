//! Params for different endpoints
use crate::messages::common_fields::{FileId, FileUploadToken, SessionId};
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

/// Private pin to authenticate
#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
pub struct Pin(String);

#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
pub struct PrepareDownloadParams {
    pin: Pin,
}

#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DownloadParams {
    session_id: SessionId,
    file_id: FileId,
}

#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
pub struct PrepareUploadParams {
    pin: Pin,
}

#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UploadParams {
    session_id: SessionId,
    file_id: FileId,
    token: FileUploadToken,
}
