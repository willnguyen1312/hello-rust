// Read file sammple.ts using fs module
// Print the content of the file
// import fs from "fs";

const fs = require("fs");

fs.readFile("./sample.txt", "utf8", (_, data) => {
  console.log(data);
});
