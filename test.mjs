import { readFileAsync } from "./index.js";

console.log("From native", (await readFileAsync("./index.js")).toString());
