[![CI](https://github.com/tpisto/ftp-any-get/actions/workflows/CI.yml/badge.svg)](https://github.com/tpisto/ftp-any-get/actions/workflows/CI.yml)

# ftp-any-get
Node.js native module to get file from FTP, FTPS and SFTP sources.

## Motivation
Easy to install. Easy to use. Just single asynchronous function: "getFile".

## Install
```
npm install @tpisto/ftp-any-get
```
or 
```
npm add @tpisto/ftp-any-get
```

## How to use

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
