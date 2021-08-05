use async_recursion::async_recursion;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

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
pub async fn get_dir(url: String, client: &Client) -> Result<(), Box<dyn Error>> {
    let res = client
        .get(url)
        .header("User-Agent", "request")
        .send()
        .await?
        .json::<ApiData>()
        .await?;

    for obj in res {
        if obj.api_datum_type == "dir" {
            get_dir(obj.links.links_self, &client).await?;
        } else {
            println!("{}", obj.name);
        }
    }

    Ok(())
}
