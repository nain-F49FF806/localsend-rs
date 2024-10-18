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

use super::{
    common_fields::FilesInfoMap,
    common_fields::{DeviceInfo, PreferDownload, SessionId},
};

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
struct PreDownloadMeta {
    info: PreDownloadInfo,
    session_id: SessionId,
    files: FilesInfoMap,
}

struct PreDownloadInfo {
    device_info: DeviceInfo,
    download: PreferDownload,
}
