use std::{io::{stdout, Write}, process::Command};

fn zola_build() -> Result<(), std::io::Error> {
    const NAME: &str = "zola";
    const ARG: &str = "build";
    let out = Command::new(NAME).arg(ARG).output()?;
    println!("Status: {}", out.status);
    stdout().write(&out.stdout)?;
    stdout().write(&out.stderr)?;
    stdout().flush()?;
    if out.status.success() {
        Ok(())
    }
    else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, format!("non zero exit status: {}", out.status).as_str()))
    }
}

fn main() -> Result<(), std::io::Error> {
    zola_build()?;
    Ok(())
}
