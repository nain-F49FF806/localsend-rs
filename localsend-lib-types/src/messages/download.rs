//! Reverse File Transfer (HTTP) aka Download API
//!
//! This is an alternative method which should be used when LocalSend is not available on the receiver.
//!
//! The sender setups an HTTP server to send files to other members by providing a URL.
//!
//! The receiver then opens the browser with the given URL and downloads the file.
//!
//! It is important to note that the unencrypted HTTP protocol is used because browsers reject self-signed certificates.
//! 5.1 Browser URL
//!
//! The receiver can open the following URL in the browser to download the file.
//!
//! `http://<sender-ip>:<sender-port>``
//!
//!

use derive_getters::Getters;
use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

use super::common_fields::{DeviceInfo, FilesInfoMap, SessionId, Version};

/// 5.2 Receive Request (Metadata Only)
///
/// Send to the sender a request to get a list of file metadata.
///
/// The downloader may add ?sessionId=mySessionId. In this case, the request should be accepted if it is the same session.
///
/// This is needed if the user refreshes the browser page.
///
/// If a PIN is required, the query parameter ?pin=123456 should be added.
///
/// `POST /api/localsend/v2/prepare-download`
///
/// Request
/// ```json
/// No body
/// ```
/// Response
///
/// ```json
/// {
///   "info": {
///     "alias": "Nice Orange",
///     "version": "2.0",
///     "deviceModel": "Samsung", // nullable
///     "deviceType": "mobile", // mobile | desktop | web | headless | server, nullable
///     "fingerprint": "random string", // ignored in HTTPS mode
///     "download": true, // if the download API (5.2 and 5.3) is active (optional, default: false)
///   },
///   "sessionId": "mySessionId",
///   "files": {
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
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq, Getters)]
#[serde(rename_all = "camelCase")]
pub struct PrepareDownloadResponse {
    info: PrepareDownloadMeta,
    session_id: SessionId,
    files: FilesInfoMap,
}

#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq, Getters)]
struct PrepareDownloadMeta {
    version: Version,
    #[serde(flatten)]
    device_info: DeviceInfo,
    download: serde_bool::True,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use crate::messages::common_fields::{DeviceInfo, DeviceType, FileId, FileInfo, FilesInfoMap};

    use super::{PrepareDownloadMeta, PrepareDownloadResponse};

    #[test]
    fn predownload_meta_deserialize_serialize() {
        let response_json = json!(
            {
              "info": {
                "alias": "Nice Orange",
                "version": "2.0",
                "deviceModel": "Samsung", // nullable
                "deviceType": "mobile", // mobile | desktop | web | headless | server, nullable
                "fingerprint": "random string", // ignored in HTTPS mode
                "download": true, // if the download API (5.2 and 5.3) is active (optional, default: false)
              },
              "sessionId": "mySessionId",
              "files": {
                "some file id": {
                  "id": "some file id",
                  "fileName": "my image.png",
                  "size": 324242, // bytes
                  "fileType": "image/jpeg",
                  "sha256": "*sha256 hash*", // nullable
                  "preview": "*preview data*" // nullable
                },
                "another file id": {
                  "id": "another file id",
                  "fileName": "another image.jpg",
                  "size": 1234,
                  "fileType": "image/jpeg",
                  "sha256": "*sha256 hash*",
                  "preview": "*preview data*"
                }
              }
            }
        );
        let info = PrepareDownloadMeta::new(
            "2.0".to_string().into(),
            DeviceInfo::new(
                "Nice Orange".to_string().into(),
                Some("Samsung".to_string().into()),
                DeviceType::Mobile,
                "random string".to_string().into(),
            ),
            serde_bool::True,
        );
        let mut files_map: HashMap<FileId, FileInfo> = HashMap::new();
        files_map.insert(
            "some file id".to_string().into(),
            FileInfo::new(
                "some file id".to_string().into(),
                "my image.png".to_string(),
                324242,
                "image/jpeg".to_string(),
                Some("*sha256 hash*".to_string().into()),
                Some("*preview data*".to_string().into()),
                None,
            ),
        );
        files_map.insert(
            "another file id".to_string().into(),
            FileInfo::new(
                "another file id".to_string().into(),
                "another image.jpg".to_string(),
                1234,
                "image/jpeg".to_string(),
                Some("*sha256 hash*".to_string().into()),
                Some("*preview data*".to_string().into()),
                None,
            ),
        );
        let files = FilesInfoMap::new(files_map);
        let constructed_response =
            PrepareDownloadResponse::new(info, "mySessionId".to_string().into(), files);
        let read_response = serde_json::from_value(response_json.clone()).unwrap();
        assert_eq!(constructed_response, read_response);
        let written_response = serde_json::to_value(constructed_response).unwrap();
        assert_eq!(response_json, written_response);
    }
}
