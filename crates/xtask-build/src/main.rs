use std::{
    io::{stderr, stdout, Write},
    process::Command,
};

mod github;

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

fn main() -> Result<(), std::io::Error> {
    zola_build()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::github::{latest_release_url, Release};

    const USER: &str = "paulusminus";
    const REPO: &str = "lipl-control";

    #[test]
    fn get_latest_release() {
        let url = latest_release_url(USER, REPO);
        let response = ureq::get(&url)
            .call()
            .unwrap()
            .into_json::<Release>()
            .unwrap();
        let download_url = response
            .assets
            .get(0)
            .map(|a| a.browser_download_url.as_str());
        assert_eq!(
            download_url,
            Some("https://github.com/paulusminus/lipl-control/releases/download/v1.8.3/web.tar.gz")
        );
    }
}
