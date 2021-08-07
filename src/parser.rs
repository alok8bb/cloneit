use std::error::Error;
use url::Url;

#[derive(Debug)]
pub struct Directory {
    pub root: String,
    pub branch: String,
    pub path: String,
    pub username: String,
    pub repository: String,
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

    Ok(data)
}
