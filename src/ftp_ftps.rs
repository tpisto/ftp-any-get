extern crate suppaftp;

use anyhow::Result;
use futures::AsyncReadExt;
use std::str;
use suppaftp::async_native_tls::TlsConnector;
use suppaftp::FtpStream;
use url::Url;

pub async fn get_ftp_file(url: Url, accept_invalid_certs: Option<bool>) -> Result<Vec<u8>> {
  let address = if url.port().is_some() {
    format!("{}:{}", url.host_str().unwrap(), url.port().unwrap())
  } else {
    format!("{}:21", url.host_str().unwrap())
  };

  let ftp_stream = FtpStream::connect(address).await?;

  // Switch to secure mode if needed
  let mut ftp_stream = if url.scheme() == "ftps" {
    ftp_stream
      .into_secure(
        TlsConnector::new().danger_accept_invalid_certs(accept_invalid_certs.unwrap_or(false)),
        url.host_str().unwrap(),
      )
      .await
      .unwrap()
  } else {
    ftp_stream
  };

  // Login
  if url.password().is_some() {
    ftp_stream
      .login(url.username(), url.password().unwrap())
      .await?;
  } else {
    ftp_stream.login(url.username(), "").await?;
  }

  // We need to split the path and filename so that we can first change to the right directory using FTP commands
  let path_parts: Vec<&str> = url.path().split('/').collect();
  let path = path_parts[1..path_parts.len() - 1].join("/");
  let file_name = path_parts[path_parts.len() - 1];

  // Change into a new directory, relative to the one we are currently in.
  if path.len() > 0 {
    let _ = ftp_stream.cwd(path).await?;
  }

  // Read the file
  let mut reader = ftp_stream.retr_as_stream(file_name).await?;
  let mut buffer = Vec::new();
  reader.read_to_end(&mut buffer).await?;
  ftp_stream.finalize_retr_stream(reader).await?;
  ftp_stream.quit().await?;

  Ok(buffer)
}
