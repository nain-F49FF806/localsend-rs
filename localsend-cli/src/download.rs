use std::path::{Path, PathBuf};

use crate::utils::{ask_confirm, dbgr};
use crate::{args::DownloadArgs, constants::APP_USER_AGENT};
use compounderr::compose_errors as funsie_errors;
use localsend_lib_types::messages::common_fields::FilesInfoMap;
use localsend_lib_types::messages::download::PrepareDownloadResponse;
use reqwest::Error as ReqwestError;
use reqwest::Url;
use serde_json::Error as SerdeJsonError;

/// Download files from given server using pin if required
// #[tokio::main(flavor = "current_thread")]
pub fn download(download_args: DownloadArgs) {
    let base_url = Url::parse(&format!(
        "http://{}:{}",
        download_args.sender(),
        download_args.port()
    ))
    .expect("Ipv4addr and port should be parseable");
    println!("Querying for files info");
    let prepare_download_response =
        prepare_download_request(&base_url, download_args.pin().as_deref())
            .inspect_err(dbgr)
            .unwrap();
    let session_id = prepare_download_response.session_id();
    let files_map = prepare_download_response.files();
    let destination = download_args.destination();
    dbg!(files_map);
    let confirm = ask_confirm(
        &format!("The above files will be downloaded to {destination:?}. Continue?",),
        Some(true),
    )
    .unwrap();
    if !confirm {
        return;
    };
    download_files(files_map, download_args.destination());
}

#[funsie_errors]
#[errorset(ReqwestError, SerdeJsonError)]
pub fn prepare_download_request(
    base_url: &Url,
    pin: Option<&str>,
) -> Result<PrepareDownloadResponse, _> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let pre_download_url = base_url
        .join("/api/localsend/v2/prepare-download")
        .expect("prechecked path shouldn't error");

    let pre_download_req = if let Some(pin) = pin {
        client.post(pre_download_url).query(&[("pin", pin)])
    } else {
        client.post(pre_download_url)
    };

    let response = pre_download_req.send()?.error_for_status()?;
    let response_text = response.text()?;
    let prepare_download_response = serde_json::from_str(&response_text)?;
    // dbg!(&prepare_download_response);
    Ok(prepare_download_response)
}

pub fn download_files(files_info_map: &FilesInfoMap, destination: &PathBuf) {}
