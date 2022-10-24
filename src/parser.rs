use std::error::Error;
use url::Url;

#[derive(Debug)]
pub struct Directory {
    pub root: String,
    pub branch: String,
    pub path: String,
    pub username: String,
    pub repository: String,
    pub clone_path: Option<String>,
}

pub fn parse_url(url: &str) -> Result<String, Box<dyn Error>> {
    let parsed_url = match Url::parse(url) {
        Ok(url) => url,
        Err(err) => return Err(format!("Invalid URL: {}", err.to_string()).into()),
    };

    Ok(parsed_url.path().to_string())
}

pub fn parse_path(path: &str, clone_path: Option<String>) -> Result<Directory, Box<dyn Error>> {
    let mut patterns: Vec<&str> = path.split('/').collect();
    if patterns.last().unwrap() == &"" {
        patterns.pop();
    }

    if patterns.len() < 3 {
        return Err("Error parsing URL".into());
    }

    let data = Directory {
        username: patterns[1].to_string(),
        repository: patterns[2].to_string(),
        branch: if patterns.get(4) == None {
            "".to_string()
        } else {
            patterns[4].to_string()
        },
        root: if patterns.last() == None {
            patterns[2].to_string()
        } else {
            (*patterns.last().unwrap()).to_string()
        },
        path: if patterns.get(5) == None {
            "".to_string()
        } else {
            patterns[5..]
                .iter()
                .map(|i| format!("/{}", i))
                .collect::<String>()
        },
        clone_path,
    };

    Ok(data)
}
