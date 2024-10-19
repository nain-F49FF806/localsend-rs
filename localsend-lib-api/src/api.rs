//! Params for different endpoints
use derive_more::derive::Constructor;
use localsend_lib_messages::common_fields::{FileId, FileUploadToken, SessionId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
struct Pin(String);

#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
struct PrepareDownloadParams {
    pin: Pin,
}

#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
#[serde(rename_all = "camelCase")]
struct DownloadParams {
    session_id: SessionId,
    file_id: FileId,
}

#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
struct PrepareUploadParams {
    pin: Pin,
}

#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
#[serde(rename_all = "camelCase")]
struct UploadParams {
    session_id: SessionId,
    file_id: FileId,
    token: FileUploadToken,
}
