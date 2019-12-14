const fs = require("fs");

module.exports = main;

function main() {
  fs.readFile("./input.txt", "utf8", (err, data) => {
    const orbitMap = data.split(/\r?\n/g);
    console.log(orbitMap);
  });
}
