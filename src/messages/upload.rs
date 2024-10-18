use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::common_fields::{
    DeviceInfo, FilesInfoMap, FilesTokenMap, Port, PreferDownload, Protocol, SessionId,
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
    #[serde(flatten)]
    device_info: DeviceInfo,
    port: Port,
    protocol: Protocol,
    download: Option<PreferDownload>,
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
struct PrepareUploadResponse {
    session_id: SessionId,
    files: FilesTokenMap,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use mediatype::MediaTypeBuf;
    use serde_json::json;
    use time::OffsetDateTime;

    use crate::messages::{
        common_fields::{
            DeviceInfo, DeviceType, FileId, FileInfo, FileMetadata, FilesInfoMap, PreferDownload,
            Protocol,
        },
        upload::{PrepareUploadDeviceInfo, PrepareUploadRequest},
    };

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
            DeviceInfo::new(
                "Nice Orange".to_string().into(),
                "2.0".to_string().into(),
                Some("Samsung".to_string().into()),
                DeviceType::Mobile,
                "random string".to_string().into(),
            ),
            53317.into(),
            Protocol::Https,
            Some(PreferDownload::new(true)),
        );
        let mut files_map: HashMap<FileId, FileInfo> = HashMap::new();
        files_map.insert(
            "some file id".to_string().into(),
            FileInfo::new(
                "some file id".to_string().into(),
                "my image.png".to_string(),
                324242,
                MediaTypeBuf::from_string("image/jpeg".to_string()).unwrap(),
                Some("*sha256 hash*".to_string().into()),
                Some("*preview data*".to_string().into()),
                Some(FileMetadata::new(
                    OffsetDateTime::parse(
                        "2021-01-01T12:34:56Z",
                        &time::format_description::well_known::Rfc3339,
                    )
                    .unwrap(),
                    OffsetDateTime::parse(
                        "2021-01-01T12:34:56Z",
                        &time::format_description::well_known::Rfc3339,
                    )
                    .unwrap(),
                )),
            ),
        );
        files_map.insert(
            "another file id".to_string().into(),
            FileInfo::new(
                "another file id".to_string().into(),
                "another image.jpg".to_string(),
                1234,
                MediaTypeBuf::from_string("image/jpeg".to_string()).unwrap(),
                Some("*sha256 hash*".to_string().into()),
                Some("*preview data*".to_string().into()),
                None,
            ),
        );
        let files = FilesInfoMap::new(files_map);
        let constructed_response = PrepareUploadRequest::new(info, files);
        let read_request = serde_json::from_value(request_json.clone()).unwrap();
        print!(
            "{}",
            serde_json::to_string_pretty(&constructed_response).unwrap()
        );
        assert_eq!(constructed_response, read_request);
        let written_response = serde_json::to_value(constructed_response).unwrap();
        assert_eq!(request_json, written_response);
    }
}
