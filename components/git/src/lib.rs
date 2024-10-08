use kit::{pwd, sh};
use regex::Regex;
use semver::Version;

mod hub;
use hub::Github;

pub fn extract_version(s: &str) -> String {
    let re: Regex = Regex::new(r"(?:(\d+)\.)?(?:(\d+)\.)?(?:(\d+)\.\d+)").unwrap();
    let matches = re.find(s);
    match matches {
        Some(m) => m.as_str().to_string(),
        _ => "0.0.2".to_string(),
    }
}

pub fn extract_tag(prefix: &str, s: &str) -> String {
    let f = format!(r"{}-(?:(\d+)\.)?(?:(\d+)\.)?(?:(\d+)\.\d+)(-\w+)?", prefix);
    let re: Regex = Regex::new(&f).unwrap();
    let matches = re.find(s);
    match matches {
        Some(m) => m.as_str().to_string(),
        _ => "".to_string(),
    }
}

pub fn current_version(prefix: &str) -> String {
    let cmd = format!(
        "git describe --match {}-* --tags $(git log -n1 --pretty='%h')",
        prefix
    );
    let out = sh(&cmd, &pwd());
    if out.contains("fatal") {
        String::from("0.0.1")
    } else {
        extract_version(&out)
    }
}

pub fn current_revision(dir: &str) -> String {
    let cmd_str = format!("git log -n 1 --format=%h {}", dir);
    sh(&cmd_str, dir)
}

pub fn branch_name() -> String {
    sh("git rev-parse --abbrev-ref HEAD", &pwd())
}

pub fn sha() -> String {
    sh("git rev-parse --short HEAD", &pwd())
}

pub fn maybe_semver(v: &str) -> Version {
    Version::parse(v).unwrap()
}

pub fn latest_version(prefix: &str) -> String {
    let cmd = format!("git describe --tags --abbrev=0 --match {}-*", prefix);
    let out = sh(&cmd, &pwd());
    if out.contains("fatal: No names found") {
        String::from("0.0.1")
    } else {
        extract_version(&out)
    }
}

pub fn tag_revision(tag: &str) -> String {
    let cmd = format!("git rev-parse {}", tag);
    sh(&cmd, &pwd())
}

pub fn current_repo() -> String {
    sh(
        "basename -s .git `git config --get remote.origin.url`",
        &pwd(),
    )
}

pub fn root() -> String {
    sh("git rev-parse --show-toplevel", &pwd())
}

pub async fn self_upgrade(repo: &str, token: &str) {
    let gh = Github::init(repo, token);
    let arch_os = hub::arch_os();
    let name = match arch_os.as_str() {
        "x86_64-linux" => "tc-x86_64-linux",
        "x86_64-macos" => "tc-x86_64-apple",
        "aarch64-macos" => "tc",
        _ => panic!("unknown os {}", arch_os),
    };
    gh.download_asset(name, "/tmp/tc").await;
}
