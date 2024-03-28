use std::{
    error::Error, io::{stderr, stdout, Write}, process::Command
};

const USER: &str = "paulusminus";
const REPO: &str = "lipl-control";
const DOWNLOAD_URL: &str =
    "https://github.com/paulusminus/lipl-control/releases/download/v1.8.3/web.tar.gz";

async fn octocrab_get_latest_release(user: &str, repo: &str) -> Result<String, std::io::Error> {
    let release = octocrab::instance()
        .repos(user, repo)
        .releases()
        .get_latest()
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Gihub Api"))?;
    let download_url = release
        .assets
        .get(0)
        .map(|a| a.browser_download_url.to_string());
    download_url.ok_or(std::io::Error::new(std::io::ErrorKind::NotFound, ""))
}

fn zola_build() -> Result<(), std::io::Error> {
    const NAME: &str = "zola";
    const ARG: &str = "build";
    let out = Command::new(NAME).arg(ARG).output()?;
    println!("Status: {}", out.status);
    if out.status.success() {
        stdout().write_all(&out.stdout)?;
        stdout().flush()?;
        Ok(())
    } else {
        stderr().write_all(&out.stderr)?;
        stderr().flush()?;
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("non zero exit status: {}", out.status),
        ))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), std::io::Error> {
    let release = octocrab_get_latest_release(USER, REPO).await?;
    zola_build()?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::github::{latest_release_url, Release};
    #[test]
    fn jaja() {
        assert!(true);
    }
}
