const readline = require("readline");
const fs = require("fs");
let output = 0;

const rl = readline.createInterface({
  input: fs.createReadStream("input.txt")
});

rl.on("line", mass => {
  console.log("Mass from file: ", mass);
  output += Math.floor(mass / 3) - 2;
  console.log("New Fuel: " + output);
});

console.log("[Day 1] Part 1: The first element's value is" + output);
