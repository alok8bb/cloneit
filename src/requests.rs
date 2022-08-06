use async_recursion::async_recursion;
use base64::decode;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};
use tokio::io::AsyncWriteExt;

use crate::parser::Directory;

pub type ApiData = Vec<ApiObject>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiObject {
    name: String,
    path: String,
    sha: String,
    size: i64,
    url: String,
    content: Option<String>,
    download_url: Option<String>,
    #[serde(rename = "type")]
    object_type: String, // dir or file
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

pub async fn fetch_data(data: &Directory) -> Result<(), Box<dyn Error>> {
    let url = if data.path.is_empty() {
        format!(
            "https://api.github.com/repos/{}/{}/contents/",
            data.username, data.repository
        )
    } else {
        format!(
            "https://api.github.com/repos/{}/{}/contents{}?ref={}",
            data.username, data.repository, data.path, data.branch
        )
    };

    let client = Client::new();
    let path = format!("./{}", data.root);
    fs::create_dir(&path)?;

    get_dir(&url, &client, Path::new(&path)).await?;

    Ok(())
}

#[async_recursion]
pub async fn get_dir(url: &str, client: &Client, dir: &Path) -> Result<(), Box<dyn Error>> {
    let res: String = client
        .get(url)
        .header("User-Agent", "request")
        .send()
        .await?
        .text()
        .await?;
    // `TODO: add check for response codes (200, 403, 404)

    if res.starts_with('{') {
        let obj: ApiObject = serde_json::from_str(&res)?;
        println!("{:#?}", obj);
        match obj.content {
            Some(content) => {
                if content.is_empty() {
                    get_file_data(obj.download_url.unwrap(), client, obj.name, dir).await?
                } else {
                    write_data(content, dir, obj.name).await?
                }
            }
            None => (),
        };
    } else {
        let api_data: ApiData = serde_json::from_str(&res)?;

        for obj in api_data {
            if obj.object_type == "dir" {
                let dir_name = dir.join(obj.name);
                fs::create_dir(&dir_name)?;
                get_dir(&obj.links.links_self, client, dir_name.as_path()).await?;
            } else {
                // FIXME: check if content is empty or not
                let download_url = obj.download_url.unwrap();
                get_file_data(download_url, client, obj.name, dir).await?
            }
        }
    }

    Ok(())
}

async fn write_data(content: String, dir: &Path, filename: String) -> Result<(), Box<dyn Error>> {
    let content = content
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let content_str = decode(content).unwrap();
    tokio::fs::File::create(dir.join(filename))
        .await?
        .write_all(&content_str[..])
        .await?;
    Ok(())
}

pub async fn get_file_data(
    url: String,
    client: &Client,
    filename: String,
    dir: &Path,
) -> Result<(), Box<dyn Error>> {
    let download_size = {
        let resp = client.get(url.as_str()).send().await?;
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

    let mut resp = req.send().await?;
    if !resp.status().is_success() {
        return Err(format!("Couldn't download URL. Error: {:?}", resp.status(),).into());
    }

    while let Some(chunk) = resp.chunk().await? {
        progress_bar.inc(chunk.len() as u64); // Increase Progressbar

        outfile.write(&chunk).await?;
    }

    progress_bar.finish_with_message(filename);

    Ok(())
}
