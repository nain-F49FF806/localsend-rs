use std::collections::HashMap;

use mediatype::MediaTypeBuf;

use super::SHA256;

/// File ID
///
/// Unique ID for a unique file
pub(in super::super) struct FileId(String);

/// File Preview
///
/// Base64 encoded thumbnail for file
pub(in super::super) struct FilePreview(Vec<u8>);

/// File Info
///
/// File metadata
pub(in super::super) struct FileInfo {
    id: FileId,
    file_name: String,
    size: u64,
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
pub(in super::super) struct FilesInfoMap(HashMap<FileId, FileInfo>);
