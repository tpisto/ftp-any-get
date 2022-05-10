use anyhow::Result;
use std::path::Path;
use std::str;

use url::{ParseError, Url};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_ssh2::AsyncSession;

pub async fn get_sftp_file(url: Url) -> Result<Vec<u8>> {
  let address = format!("{}:{}", url.host_str().unwrap(), url.port().unwrap_or(22));

  let tcp = std::net::TcpStream::connect(address)?;
  let mut session = AsyncSession::new(tcp)?;
  session.handshake().await?;
  session
    .userauth_password(url.username(), url.password().unwrap_or(""))
    .await?;

  let sftp = session.sftp().await?;
  let mut my_file = sftp.open(Path::new(url.path())).await?;
  let mut buffer = Vec::new();
  my_file.read_to_end(&mut buffer).await?;

  Ok(buffer)
}
