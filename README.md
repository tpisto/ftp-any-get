[![CI](https://github.com/tpisto/ftp-any-get/actions/workflows/CI.yml/badge.svg)](https://github.com/tpisto/ftp-any-get/actions/workflows/CI.yml)

# ftp-any-get
Node.js native module to get file from FTP, FTPS and SFTP sources.

## Motivation 🧐
Easy to install. Easy to use. Just single asynchronous function: "getFile".

## How to use 📚

Get file from the ftp server
```javascript
import { getFile } from "@tpisto/ftp-any-get"

// Fetch from FTP server
let ftpFile = await getFile("ftp://demo:password@my-ftp-server.net/my-file.txt");

// Fetch from FTP server using TLS
let ftpsFile = await getFile("ftps://demo:password@my-ftp-server.net/my-file.txt");

// Fetch file using SFTP. SFTP runs over the SSH protocol.
let sftpFile = await getFile("sftp://demo:password@my-ftp-server.net/my-file.txt");
```

## Install ☁
```
npm install @tpisto/ftp-any-get
```
or 
```
npm add @tpisto/ftp-any-get
```

## Supported platforms (macOS M1 and x86_64, Linux ARM and x86_64)

- x86_64-apple-darwin
- aarch64-apple-darwin
- x86_64-unknown-linux-gnu
- aarch64-unknown-linux-gnu

### Install notes (Ubuntu/Debian)
⚠️ You need to have "ca-certificates" installed in the system. For example "node:16" docker container has that already, but "node:16-slim" does not. So in "slim" containers you need to install "ca-certificates" package. If you do not have "ca-certificates" in your system, you can use "FTP" and "SFTP", but "FTPS" will fail with error ```thread 'tokio-runtime-worker' panicked at 'called `Result::unwrap()` on an `Err` value: SecureError("error:1416F086:SSL routines:tls_process_server_certificate:certificate verify failed:ssl/statem/statem_clnt.c:1914: (unable to get local issuer certificate)")', src/ftp_ftps.rs:24:8```

## This Node.js native module is written in Rust. Powered by:



- <img src="https://napi.rs/img/favicon.png" width="20" /> [napi.rs](https://napi.rs/)
- [suppaftp](https://github.com/veeso/suppaftp)
- [tokio-ssh2](https://github.com/tyan-boot/tokio-ssh2)
