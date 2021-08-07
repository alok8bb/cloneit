use async_recursion::async_recursion;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{error::Error, fs};
use tokio::io::AsyncWriteExt;

pub type ApiData = Vec<ApiOjbect>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiOjbect {
    name: String,
    path: String,
    sha: String,
    size: i64,
    url: String,
    html_url: String,
    git_url: String,
    download_url: Option<String>,
    #[serde(rename = "type")]
    api_datum_type: String,
    #[serde(rename = "_links")]
    links: Links,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    #[serde(rename = "self")]
    links_self: String,
    git: String,
    html: String,
}

#[async_recursion]
pub async fn get_dir(url: String, client: &Client, dir: &Path) -> Result<(), Box<dyn Error>> {
    let res: String = client
        .get(url)
        .header("User-Agent", "request")
        .send()
        .await?
        .text()
        .await?;

    // Check if single file was given (download directly)
    if res.starts_with("{") {
        let ojb: ApiOjbect = serde_json::from_str(&res)?;

        get_filedata(ojb.download_url.unwrap(), client, ojb.name, dir).await?;
    } else {
        let api_data: ApiData = serde_json::from_str(&res)?;

        for obj in api_data {
            if obj.api_datum_type == "dir" {
                let dir_name = dir.join(obj.name);
                fs::create_dir(&dir_name)?;
                get_dir(obj.links.links_self, &client, dir_name.as_path()).await?;
            } else {
                let download_url = obj.download_url.unwrap();
                get_filedata(download_url, client, obj.name, dir).await?;
            }
        }
    }

    Ok(())
}

pub async fn get_filedata(
    url: String,
    client: &Client,
    filename: String,
    dir: &Path,
) -> Result<(), Box<dyn Error>> {
    let download_size = {
        let resp = client.head(url.as_str()).send().await?;
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(
                format!("Couldn't download URL: {}. Error: {:?}", url, resp.status(),).into(),
            );
        }
    };

    let req = client.get(url);
    let progress_bar = ProgressBar::new(download_size);

    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:60.cyan/blue}] {bytes}/{total_bytes} {msg}")
            .progress_chars("- C * "),
    );
    progress_bar.set_message(filename.clone());

    let mut outfile = tokio::fs::File::create(dir.join(filename.clone())).await?;

    let mut download = req.send().await?;
    while let Some(chunk) = download.chunk().await? {
        progress_bar.inc(chunk.len() as u64); // Increase Progressbar

        outfile.write(&chunk).await?;
    }

    progress_bar.finish_with_message(filename);

    Ok(())
}
