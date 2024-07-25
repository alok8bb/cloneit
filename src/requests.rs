use crate::parser::Directory;
use async_recursion::async_recursion;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{error::Error, path::Path};
use tokio::io::AsyncWriteExt;
use yansi::Paint;

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
                    if !Path::new(p).exists() {
                        tokio::fs::create_dir(p).await?;
                    }

                    let path = Path::new(p);
                    write_file(object, &path, &client).await?;
                }
                None => {
                    write_file(object, &path, &client).await?;
                }
            }

            // write_file(object, &path, &client).await?;
        }

        // Check if given URL is directory and crate root directory based on that
        // This solves creating unneccessary directory problem even if there was only one file
        ApiResponse::Array(_) => {
            match clone_path.as_deref() {
                Some(".") => {
                    get_dir(&url, &client, &path).await?;
                }
                Some(p) => {
                    if !Path::new(p).exists() {
                        tokio::fs::create_dir(p).await?;
                    }
                    let path = Path::new(p);
                    get_dir(&url, &client, &path).await?;
                }
                None => {
                    let next_path = path.join(&project_root); // creates root dir
                    tokio::fs::create_dir(&next_path).await?;

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
                    tokio::fs::create_dir(&next_path).await?;
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

            let mut outfile = tokio::fs::File::create(&new_path).await?;
            while let Some(chunk) = res.chunk().await? {
                outfile.write(&chunk).await?;
            }

            Ok(())
        }
        None => return Err(format!("Could not get the download link!").into()),
    }
}
