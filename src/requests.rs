use crate::parser::Directory;
use async_recursion::async_recursion;
use kdam::{term::Colorizer, tqdm, BarExt, Column, RichProgress};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::{error::Error, path::Path};
use tokio::io::AsyncWriteExt;

pub type ApiData = Vec<ApiObject>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ApiResponse {
    Object(ApiObject),
    Array(ApiData),
    Message(ApiMessage),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiMessage {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiObject {
    name: String,
    path: String,
    url: String,
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

    download(&url, &data.root, &data.clone_path).await?;

    Ok(())
}

async fn build_request(url: &str, client: &Client) -> Result<ApiResponse, Box<dyn Error>> {
    let res: String = client
        .get(url)
        .header("User-Agent", "request")
        .send()
        .await?
        .text()
        .await?;

    match serde_json::from_str(&res) {
        Ok(val) => {
            match val {
                ApiResponse::Message(msg_object) => return Err(msg_object.message.into()),
                _ => (),
            }
            Ok(val)
        }
        Err(_) => Err(format!("Error parsing api object, check the provided url").into()),
    }
}

async fn download(
    url: &str,
    project_root: &str,
    clone_path: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let path = Path::new("./");

    let response = build_request(&url, &client).await?;

    match response {
        ApiResponse::Object(object) => {
            // single object is always a file
            match clone_path {
                Some(p) => {
                    let path = Path::new(p);

                    if !path.exists() {
                        tokio::fs::create_dir(path).await?;
                    }

                    write_file(object, &path, &client).await?;
                }
                None => {
                    write_file(object, &path, &client).await?;
                }
            }

            // write_file(object, &path, &client).await?;
        }

        // Check if given URL is directory and create root directory based on that
        // This solves creating unneccessary directory problem even if there was only one file
        ApiResponse::Array(_) => {
            match clone_path.as_deref() {
                Some(".") => {
                    get_dir(&url, &client, &path).await?;
                }
                Some(p) => {
                    let path = Path::new(p);

                    if !path.exists() {
                        tokio::fs::create_dir(path).await?;
                    }

                    get_dir(&url, &client, &path).await?;
                }
                None => {
                    let next_path = path.join(&project_root); // creates root dir
                    if !next_path.exists() {
                        tokio::fs::create_dir(&next_path).await?;
                    }

                    // recursive directory download starts here
                    get_dir(&url, &client, &next_path).await?;
                }
            }
        }

        ApiResponse::Message(_) => (),
    }

    Ok(())
}

#[async_recursion]
async fn get_dir(url: &str, client: &Client, path: &Path) -> Result<(), Box<dyn Error>> {
    let resp = build_request(url, client).await?;

    match resp {
        ApiResponse::Object(obj) => {
            write_file(obj, &path, &client).await?;
        }

        ApiResponse::Array(arr) => {
            for obj in arr {
                if obj.object_type == "dir" {
                    let next_path = path.join(obj.name);
                    if !next_path.exists() {
                        tokio::fs::create_dir(&next_path).await?;
                    }
                    get_dir(&obj.url, &client, &next_path).await?;
                } else {
                    write_file(obj, &path, &client).await?;
                }
            }
        }

        ApiResponse::Message(_) => (),
    }

    Ok(())
}

async fn write_file(
    obj: ApiObject,
    root_path: &Path,
    client: &Client,
) -> Result<(), Box<dyn Error>> {
    match &obj.download_url {
        Some(download_url) => {
            let new_path = root_path.join(&obj.name);

            let mut res = client.get(download_url).send().await?;
            let download_size = {
                if res.status().is_success() {
                    res.headers()
                        .get(header::CONTENT_LENGTH)
                        .and_then(|ct_len| ct_len.to_str().ok())
                        .and_then(|ct_len| ct_len.parse().ok())
                        .unwrap_or(0)
                } else {
                    return Err(format!(
                        "Couldn't download file from URL\nError: {}",
                        res.status()
                    )
                    .into());
                }
            };

            let mut pb = RichProgress::new(
                tqdm!(
                    total = download_size,
                    unit_scale = true,
                    unit_divisor = 1024,
                    unit = "B"
                ),
                vec![
                    Column::text("[bold blue]?"),
                    Column::Bar,
                    Column::Percentage(1),
                    Column::text("•"),
                    Column::CountTotal,
                    Column::text("•"),
                    Column::Rate,
                ],
            );

            pb.replace(0, Column::text(&format!("[bold blue]{}", &obj.name)));

            let mut outfile = tokio::fs::File::create(&new_path).await?;
            let mut downloaded = 0;
            while let Some(chunk) = res.chunk().await? {
                downloaded += chunk.len();
                pb.update_to(downloaded);
                outfile.write(&chunk).await?;
            }

            pb.write(format!("downloaded {}", &obj.name).colorize("green"));
            // pb.clear();
            Ok(())
        }
        None => return Err(format!("Could not get the download link!").into()),
    }
}
