import test from "ava";

import { getFile } from "../index.js";

test("fetch file using FTP", async (t) => {
  let file = (await getFile("ftp://demo:password@test.rebex.net/readme.txt")).toString();
  t.is(file.length, 405);
});

test("fetch file using FTPS", async (t) => {
  let file = (await getFile("ftps://demo:password@test.rebex.net/readme.txt")).toString();
  t.is(file.length, 405);
});

test("fetch file using SFTP", async (t) => {
  let file = (await getFile("sftp://demo:password@test.rebex.net/readme.txt")).toString();
  t.is(file.length, 405);
});
