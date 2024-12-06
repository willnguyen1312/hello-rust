// Read file sammple.ts using fs module
// Print the content of the file
// import fs from "fs";

const fs = require("fs");

fs.readFile("./sample.txt", "utf8", (_, data) => {
  console.log(data);
});

// last_commit_message=$(git log -1 --pretty=%B) && git reset HEAD~1 && git add . && git commit -m "$last_commit_message" && git push -f
