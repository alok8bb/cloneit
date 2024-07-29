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

impl Directory {
    pub fn new(url: Url, clone_path: Option<String>) -> Self {
        let mut segments = url.path_segments().expect("URL has path segments");

        let username = segments.next().expect("URL contains username").to_string();
        let repository = segments
            .next()
            .expect("URL contains repository")
            .to_string();
        let branch = segments.nth(1).unwrap_or("").to_string();
        let path = segments.collect::<Vec<_>>().join("/");

        Directory {
            username,
            repository: repository.clone(),
            branch,
            root: repository.clone(),
            path,
            clone_path,
        }
    }
}
