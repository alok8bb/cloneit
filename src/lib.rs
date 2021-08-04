use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

#[derive(Debug)]
pub struct Directory {
    root: String,
    branch: String,
    path: String,
    username: String,
    repository: String,
}

pub type ApiData = Vec<ApiDatum>;

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    links_self: String,
    git: String,
    html: String,
}

pub fn parse_url(url: &str) -> Result<String, Box<dyn Error>> {
    let parsed_url = Url::parse(url);
    let parsed_url = match parsed_url {
        Ok(url) => url,
        Err(_) => return Err("Error parsing URL".into()),
    };

    Ok(parsed_url.path().to_string())
}

pub fn parse_path(path: &str) -> Result<Directory, Box<dyn Error>> {
    let mut patterns: Vec<&str> = path.split('/').collect();
    if patterns.last().unwrap() == &"" {
        patterns.pop();
    }

    if patterns.len() < 3 {
        return Err("Error parsing URL".into());
    }

    println!("{:?} {}", patterns, patterns.len());
    let data = Directory {
        username: patterns[1].to_string(),
        repository: patterns[2].to_string(),
        branch: if patterns.get(4) != None {
            patterns[4].to_string()
        } else {
            "".to_string()
        },
        root: if patterns.last() != None {
            patterns.last().unwrap().to_string()
        } else {
            patterns[2].to_string()
        },
        path: if patterns.get(5) != None {
            patterns[5..]
                .into_iter()
                .map(|i| format!("/{}", i))
                .collect::<String>()
        } else {
            "".to_string()
        },
    };

    println!("{:#?}", data);

    Ok(data)
}

pub async fn fetch_data(data: Directory) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/",
        data.username, data.repository
    );
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("User-Agent", "request")
        .send()
        .await?
        .json::<ApiData>()
        .await?;

    for obj in res {
        println!("{}", obj.size);
    }
    Ok(())
}
