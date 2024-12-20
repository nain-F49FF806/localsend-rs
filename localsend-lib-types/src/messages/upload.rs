use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common_fields::{
    DeviceInfo, FilesInfoMap, FilesTokenMap, Port, Protocol, SessionId, Version,
};

/// Upload request (Metadata Only)
///
/// Sends only the metadata to the receiver.
///
/// The receiver will decide if this request gets accepted, partially accepted or rejected.
///
/// If a PIN is required, the query parameter ?pin=123456 should be added.
///
/// `POST /api/localsend/v2/prepare-upload`
///
/// Request
///
/// ```json
/// {
///   "info": {
///     "alias": "Nice Orange",
///     "version": "2.0", // protocol version (major.minor)
///     "deviceModel": "Samsung", // nullable
///     "deviceType": "mobile", // mobile | desktop | web | headless | server, nullable
///     "fingerprint": "random string", // ignored in HTTPS mode
///     "port": 53317,
///     "protocol": "https", // http | https
///     "download": true, // if the download API (5.2 and 5.3) is active (optional, default: false)
///   },
///   "files": {
///     "some file id": {
///       "id": "some file id",
///       "fileName": "my image.png",
///       "size": 324242, // bytes
///       "fileType": "image/jpeg",
///       "sha256": "*sha256 hash*", // nullable
///       "preview": "*preview data*", // nullable
///       "metadata": { // nullable
///         "modified": "2021-01-01T12:34:56Z", // nullable
///         "accessed": "2021-01-01T12:34:56Z", // nullable
///       }
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
#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
#[serde(rename_all = "camelCase")]
struct PrepareUploadRequest {
    info: PrepareUploadDeviceInfo,
    files: FilesInfoMap,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
struct PrepareUploadDeviceInfo {
    version: Version,
    #[serde(flatten)]
    device_info: DeviceInfo,
    port: Port,
    protocol: Protocol,
    download: Option<bool>,
}

/// Response
///
/// ```json
/// {
///   "sessionId": "mySessionId",
///   "files": {
///     "someFileId": "someFileToken",
///     "someOtherFileId": "someOtherFileToken"
///   }
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Constructor, PartialEq)]
#[serde(rename_all = "camelCase")]
struct PrepareUploadResponse {
    session_id: SessionId,
    files: FilesTokenMap,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use crate::messages::{
        common_fields::{
            DeviceInfo, DeviceType, FileId, FileInfo, FileMeta, FilesInfoMap, FilesTokenMap,
            Protocol,
        },
        upload::{PrepareUploadDeviceInfo, PrepareUploadRequest},
    };

    use super::PrepareUploadResponse;

    #[test]
    fn prepareupload_request_deserialize_serialize() {
        let request_json = json!(
            {
              "info": {
                "alias": "Nice Orange",
                "version": "2.0", // protocol version (major.minor)
                "deviceModel": "Samsung", // nullable
                "deviceType": "mobile", // mobile | desktop | web | headless | server, nullable
                "fingerprint": "random string", // ignored in HTTPS mode
                "port": 53317,
                "protocol": "https", // http | https
                "download": true, // if the download API (5.2 and 5.3) is active (optional, default: false)
              },
              "files": {
                "some file id": {
                  "id": "some file id",
                  "fileName": "my image.png",
                  "size": 324242, // bytes
                  "fileType": "image/jpeg",
                  "sha256": "*sha256 hash*", // nullable
                  "preview": "*preview data*", // nullable
                  "metadata": { // nullable
                    "modified": "2021-01-01T12:34:56Z", // nullable
                    "accessed": "2021-01-01T12:34:56Z", // nullable
                  }
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

        let info = PrepareUploadDeviceInfo::new(
            "2.0".to_string().into(),
            DeviceInfo::new(
                "Nice Orange".to_string().into(),
                Some("Samsung".to_string().into()),
                DeviceType::Mobile,
                "random string".to_string().into(),
            ),
            53317.into(),
            Protocol::Https,
            Some(true),
        );
        let mut files_map: HashMap<FileId, FileInfo> = HashMap::new();
        let mut file_extra_meta: HashMap<String, String> = HashMap::new();
        file_extra_meta.insert("modified".to_string(), "2021-01-01T12:34:56Z".to_string());
        file_extra_meta.insert("accessed".to_string(), "2021-01-01T12:34:56Z".to_string());
        files_map.insert(
            "some file id".to_string().into(),
            FileInfo::new(
                "some file id".to_string().into(),
                "my image.png".to_string(),
                324242,
                "image/jpeg".to_string(),
                Some("*sha256 hash*".to_string().into()),
                Some("*preview data*".to_string().into()),
                Some(FileMeta::new(file_extra_meta)),
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
        let files: FilesInfoMap = FilesInfoMap::new(files_map);
        let constructed_request = PrepareUploadRequest::new(info, files);
        let read_request = serde_json::from_value(request_json.clone()).unwrap();
        print!(
            "{}",
            serde_json::to_string_pretty(&constructed_request).unwrap()
        );
        assert_eq!(constructed_request, read_request);
        let written_request = serde_json::to_value(constructed_request).unwrap();
        assert_eq!(request_json, written_request);
    }

    #[test]
    fn prepareupload_response_deserialize_serialize() {
        let response_json = json!(
            {
              "sessionId": "mySessionId",
              "files": {
                "someFileId": "someFileToken",
                "someOtherFileId": "someOtherFileToken"
              }
            }
        );
        let mut token_map = HashMap::new();
        token_map.insert(
            "someFileId".to_string().into(),
            "someFileToken".to_string().into(),
        );
        token_map.insert(
            "someOtherFileId".to_string().into(),
            "someOtherFileToken".to_string().into(),
        );

        let constructed_response = PrepareUploadResponse::new(
            "mySessionId".to_string().into(),
            FilesTokenMap::new(token_map),
        );
        let read_response = serde_json::from_value(response_json.clone()).unwrap();
        assert_eq!(constructed_response, read_response);
        let written_response = serde_json::to_value(constructed_response).unwrap();
        assert_eq!(response_json, written_response);
    }
}
