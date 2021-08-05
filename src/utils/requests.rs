use async_recursion::async_recursion;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{error::Error, fs};

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
        .json::<ApiData>()
        .await?;

    for obj in res {
        if obj.api_datum_type == "dir" {
            let dir_name = dir.join(obj.name);
            fs::create_dir(&dir_name)?;
            get_dir(obj.links.links_self, &client, dir_name.as_path()).await?;
        } else {
            let mut file = File::create(dir.join(obj.name))?;
            let file_content = get_filedata(obj.download_url.unwrap()).await?;
            file.write_all(file_content.as_bytes())?;
        }
    }

    Ok(())
}

pub async fn get_filedata(url: String) -> Result<String, Box<dyn Error>> {
    let res = reqwest::get(url).await?.text().await?;
    Ok(res)
}
