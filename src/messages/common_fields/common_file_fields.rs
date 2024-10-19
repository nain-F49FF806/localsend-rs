use std::collections::HashMap;

use derive_more::derive::{Constructor, From};
use mediatype::MediaTypeBuf;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};

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

/// File (extra) metadata
///
/// Optional unstructured file metadata like accessed/modified time etc
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
pub(in super::super) struct FileMeta(HashMap<String, String>);

/// File Info
///
/// essential
#[serde_as]
#[skip_serializing_none]
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
    metadata: Option<FileMeta>,
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

/// File upload token
///
/// Token required to upload each file during send using upload api
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
pub(in super::super) struct FileUploadToken(String);

/// File Token Map
///
/// ```json
/// {
///     "someFileId": "someFileToken",
///     "someOtherFileId": "someOtherFileToken"
///  }
/// ```
#[derive(Debug, Serialize, Deserialize, Constructor, From, PartialEq)]
pub(in super::super) struct FilesTokenMap(HashMap<FileId, FileUploadToken>);
