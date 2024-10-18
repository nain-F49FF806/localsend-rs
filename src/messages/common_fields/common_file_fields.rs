use std::collections::HashMap;

use derive_more::derive::{Constructor, From};
use mediatype::MediaTypeBuf;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use super::SHA256;

/// File ID
///
/// Unique ID for a unique file
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq, Eq, Hash)]
pub(in super::super) struct FileId(String);

/// File Preview
///
/// Base64 encoded thumbnail for file
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
// pub(in super::super) struct FilePreview(Vec<u8>);
pub(in super::super) struct FilePreview(String);

/// File Info
///
/// File metadata
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(in super::super) struct FileInfo {
    id: FileId,
    file_name: String,
    size: u64,
    #[serde_as(as = "DisplayFromStr")]
    file_type: MediaTypeBuf,
    sha_256: Option<SHA256>,
    preview: Option<FilePreview>,
}

/// Files info map
///
///
///   ```json
/// {
///     "some file id": {
///       "id": "some file id",
///       "fileName": "my image.png",
///       "size": 324242, // bytes
///       "fileType": "image/jpeg",
///       "sha256": "*sha256 hash*", // nullable
///       "preview": "*preview data*" // nullable
///     },
///     "another file id": {
///       "id": "another file id",
///       "fileName": "another image.jpg",
///       "size": 1234,
///       "fileType": "image/jpeg",
///       "sha256": "*sha256 hash*",
///       "preview": "*preview data*"
///     }
///   }
/// ```
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
pub(in super::super) struct FilesInfoMap(HashMap<FileId, FileInfo>);
