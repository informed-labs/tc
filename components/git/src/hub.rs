use kit as u;
use kit::*;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;

pub fn arch_os() -> String {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    format!("{}-{}", arch, os)
}

pub fn replace_exe(new_path: &str) {
    self_replace::self_replace(&new_path).unwrap();
    fs::remove_file(&new_path).unwrap();
}

#[derive(Deserialize, Clone, Debug)]
struct Asset {
    id: u64,
    browser_download_url: String,
    size: f64,
    name: String,
}

impl Asset {
    fn version(&self) -> String {
        nth(
            self.browser_download_url.split("/").into_iter().collect(),
            7,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Github {
    pub repo: String,
    pub token: String,
}

impl Github {
    pub fn init(repo: &str, token: &str) -> Github {
        Github {
            repo: String::from(repo),
            token: s!(token),
        }
    }

    fn headers(&self) -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert(s!("authorization"), format!("Bearer {}", self.token));
        h.insert(s!("accept"), s!("application/vnd.github+json"));
        h.insert(s!("x-github-api-version"), s!("2022-11-28"));
        h.insert(
            s!("user-agent"),
            s!("libcurl/7.64.1 r-curl/4.3.2 httr/1.4.2"),
        );
        h
    }

    fn with_headers(&self, key: &str, val: &str) -> HashMap<String, String> {
        let mut h = self.headers();
        h.insert(s!(key), s!(val));
        h
    }

    fn url(&self, path: &str) -> String {
        format!(
            "https://api.github.com/repos/informed-labs/{}{}",
            self.repo, path
        )
    }

    async fn latest_release_assets(&self) -> Vec<Asset> {
        let res = u::http_get(&self.url("/releases/latest"), self.headers()).await;
        let assets = &res["assets"];
        let ats: Vec<Asset> = serde_json::from_value(assets.clone()).unwrap();
        ats
    }

    pub async fn download_asset(&self, asset_name: &str, outfile: &str) {
        let assets = self.latest_release_assets().await;
        let headers = self.with_headers("accept", "application/octet-stream");
        for asset in assets {
            let id = &asset.id;
            if &asset.name == asset_name {
                println!(
                    "Upgrading to {} ({}) ref:{}",
                    &asset.version(),
                    file_size_human(asset.size),
                    id
                );
                let path = format!("/releases/assets/{}", id);
                u::download(&self.url(&path), headers.clone(), outfile).await;
                replace_exe(outfile);
                println!("Everything you can imagine is real - Pablo Picasso");
            }
        }
    }
}
