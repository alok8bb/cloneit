use async_recursion::async_recursion;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{error::Error, fs};
use tokio::io::AsyncWriteExt;

pub type ApiData = Vec<ApiDatum>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiDatum {
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
    let res = client
        .get(url)
        .header("User-Agent", "request")
        .send()
        .await?
        .text()
        .await?;

    if res.starts_with("{") {
        let data: ApiDatum = serde_json::from_str(&res)?;

        get_filedata(data.download_url.unwrap(), client, data.name, dir).await?;
    } else {
        let res_data: ApiData = serde_json::from_str(&res)?;

        for obj in res_data {
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
    file_name: String,
    dir: &Path,
) -> Result<(), Box<dyn Error>> {
    let download_size = {
        let resp = client.head(url.as_str()).send().await?;
        if resp.status().is_success() {
            resp.headers() // Gives is the HeaderMap
                .get(header::CONTENT_LENGTH) // Gives us an Option containing the HeaderValue
                .and_then(|ct_len| ct_len.to_str().ok()) // Unwraps the Option as &str
                .and_then(|ct_len| ct_len.parse().ok()) // Parses the Option as u64
                .unwrap_or(0) // Fallback to 0
        } else {
            // We return an Error if something goes wrong here
            return Err(
                format!("Couldn't download URL: {}. Error: {:?}", url, resp.status(),).into(),
            );
        }
    };

    let req = client.get(url);
    let progress_bar = ProgressBar::new(download_size);

    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:60.cyan/blue}] {bytes}/{total_bytes} - {wide_msg}")
            .progress_chars("- C * "),
    );
    progress_bar.set_message(file_name.clone());

    let mut outfile = tokio::fs::File::create(dir.join(file_name.clone())).await?;

    let mut download = req.send().await?;
    while let Some(chunk) = download.chunk().await? {
        progress_bar.inc(chunk.len() as u64); // Increase ProgressBar by chunk size

        outfile.write(&chunk).await?; // Write chunk to output file
    }

    progress_bar.finish_with_message(file_name);

    Ok(())
}
