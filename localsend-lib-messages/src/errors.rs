// use std::num::ParseIntError;

use thiserror::Error;

// #[derive(Debug, Error)]
// pub enum VersionDecodeError {
//     #[error("Version string (x.y): components were not both integers")]
//     ParseIntError(#[from] ParseIntError),
//     #[error("Version string (x.y): could not find the dot")]
//     NoDotFound,
// }

#[derive(Debug, Error)]
pub enum DownloadRequestError {
    #[error("PIN required / Invalid PIN")]
    Http401,
    #[error("Rejected")]
    Http403,
    #[error("Too many requests")]
    Http429,
    #[error("Unknown error by sender")]
    Http500,
}

#[derive(Debug, Error)]
pub enum UploadRequestError {
    #[error("Finished (No file transfer needed)")]
    Http204,
    #[error("Invalid body")]
    Http400,
    #[error("PIN required / Invalid PIN")]
    Http401,
    #[error("Rejected")]
    Http403,
    #[error("Blocked by another session")]
    Http409,
    #[error("Too many requests")]
    Http429,
    #[error("Unknown error by receiver")]
    Http500,
}

#[derive(Debug, Error)]
pub enum UploadError {
    #[error("Missing parameters")]
    Http400,
    #[error("Invalid token or IP address")]
    Http403,
    #[error("Blocked by another session")]
    Http409,
    #[error("Unknown error by receiver")]
    Http500,
}
