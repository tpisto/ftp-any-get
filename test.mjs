import { getFile } from "./index.js";

console.log("From native", (await getFile("sftp://demo:password@test.rebex.net/readme.txt")).toString());
