use anyhow;
use futures::prelude::*;
use napi::bindgen_prelude::*;
use napi::tokio::{self, fs};
use napi_derive::napi;
use url::{ParseError, Url};
extern crate openssl_probe;

mod ftp_ftps;
mod sftp;

pub async fn get_any_ftp_file(path: &str) -> anyhow::Result<Vec<u8>> {
  let ftp_url = Url::parse(path)?;

  match ftp_url.scheme() {
    "ftp" => ftp_ftps::get_ftp_file(ftp_url).await,
    "ftps" => ftp_ftps::get_ftp_file(ftp_url).await,
    "sftp" => sftp::get_sftp_file(ftp_url).await,
    _ => return Err(anyhow::anyhow!("Unsupported scheme")),
  }
}

#[napi]
async fn get_file(url: String) -> Result<Buffer> {
  openssl_probe::init_ssl_cert_env_vars();
  let file = get_any_ftp_file(url.as_str()).await;

  // Convert the result to a Buffer if result ok and return napi::Error if error
  match file {
    Ok(content) => Ok(content.into()),
    Err(e) => Err(Error::from_reason(e.to_string())),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn get_ftp_file() {
    let ftp_file = get_any_ftp_file("ftp://demo:password@test.rebex.net/readme.txt")
      .await
      .unwrap();
    assert_eq!(405, ftp_file.len());
    // Print ftp file as String
    println!("{}", String::from_utf8(ftp_file).unwrap());
  }

  #[tokio::test]
  async fn get_ftps_file() {
    let ftps_file = get_any_ftp_file("ftps://demo:password@test.rebex.net/readme.txt").await;
    assert_eq!(405, ftps_file.unwrap().len());
  }

  #[tokio::test]
  async fn get_sftp_file() {
    // let sftp_file = get_any_ftp_file("sftp://demo:password@test.rebex.net/readme.txt").await;
    let sftp_file = get_any_ftp_file("sftp://demo:password@test.rebex.net/readme.txt").await;

    assert_eq!(405, sftp_file.unwrap().len());
  }
}
