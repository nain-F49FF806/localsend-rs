use std::path::{Path, PathBuf};

use crate::args::DownloadArgs;
use crate::constants::FOX_USER_AGENT;
use crate::utils::{ask_confirm, dbgr, sanitize_relative_path};
use compounderr::compose_errors as funsie_errors;
use localsend_lib_types::messages::common_fields::{FileInfo, FilesInfoMap, SessionId};
use localsend_lib_types::messages::download::PrepareDownloadResponse;
use reqwest::Url;
use reqwest::{Client, Error as ReqwestError};
use serde_json::Error as SerdeJsonError;
use std::io::Error as IoError;
use tokio::io::AsyncWriteExt;
use url::ParseError as UrlParseError;

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
    let files_map = prepare_download_response.files();
    let session_id = prepare_download_response.session_id();
    let destination = download_args.destination();
    for (_file_id, file_info) in files_map.as_ref().iter() {
        println!(
            "{}\t{}b\t{}",
            file_info.file_type(),
            file_info.size(),
            file_info.file_name()
        )
    }
    let confirm = ask_confirm(
        &format!("The above files will be downloaded to {destination:?}. Continue?",),
        Some(true),
    )
    .unwrap();
    if !confirm {
        return;
    };
    engage_downloader(
        &base_url,
        download_args.pin().as_deref(),
        session_id,
        files_map,
        download_args.destination(),
    )
    .unwrap();
}

#[funsie_errors]
#[errorset(ReqwestError, SerdeJsonError)]
pub fn prepare_download_request(
    base_url: &Url,
    pin: Option<&str>,
) -> Result<PrepareDownloadResponse, _> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(FOX_USER_AGENT)
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

#[funsie_errors]
#[errorset(IoError)]
pub fn engage_downloader(
    base_url: &Url,
    pin: Option<&str>,
    session_id: &SessionId,
    files_info_map: &FilesInfoMap,
    destination: &Path,
) -> Result<(), _> {
    // Create the runtime
    let rt = tokio::runtime::Runtime::new()?;

    // Run the main download_files async function on the runtime
    rt.block_on(async {
        println!("Downloading files");
        download_files(base_url, pin, session_id, files_info_map, destination).await
    });

    Ok(())
}

async fn download_files(
    base_url: &Url,
    pin: Option<&str>,
    session_id: &SessionId,
    files_info_map: &FilesInfoMap,
    destination: &Path,
) {
    let client = Client::new();
    let mut joinset = tokio::task::JoinSet::new();
    let _abort_handles: Vec<_> = files_info_map
        .as_ref()
        .iter()
        .map(|(_file_id, file_info)| {
            joinset.spawn(download_silngle_file(
                client.clone(),
                base_url.clone(),
                pin.map(str::to_string),
                session_id.clone(),
                file_info.clone(),
                destination.to_path_buf(),
            ))
        })
        .collect();
    let results = joinset.join_all().await;
    println!("Results (unordered): {:?}", results);
}

/// Download given file as per localsend protocol.
/// This func could run as a separate task, maybe even on a differnt thread.
/// So we take ownership of our inputs.
#[funsie_errors]
#[errorset(UrlParseError, ReqwestError, IoError)]
async fn download_silngle_file(
    client: Client,
    base_url: Url,
    pin: Option<String>,
    session_id: SessionId,
    file_info: FileInfo,
    destination: PathBuf,
) -> Result<(), _> {
    let relative_file_path = sanitize_relative_path(file_info.file_name());
    let full_file_path = destination.join(relative_file_path);
    if let Some(parent_dir) = full_file_path.parent() {
        tokio::fs::create_dir_all(parent_dir).await?;
    };
    let file_url = base_url.join("/api/localsend/v2/download")?;
    let mut query = Vec::from([
        ("sessionId", session_id.to_string()),
        ("fileId", file_info.id().to_string()),
    ]);
    if let Some(pin) = pin {
        query.push(("pin", pin));
    };
    let mut res = client.get(file_url).query(&query).send().await?;
    res.error_for_status_ref()?;
    println!("writing {:?}", full_file_path);
    let mut open_file = tokio::fs::File::create(full_file_path).await?;
    while let Some(chunk) = res.chunk().await? {
        open_file.write_all(&chunk).await?;
    }
    Ok(())
}
