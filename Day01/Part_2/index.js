const readline = require("readline");
const fs = require("fs");
let output = 0;

const rl = readline.createInterface({
  input: fs.createReadStream("input.txt")
});

rl.on("line", mass => {
  console.log("Mass from file: ", mass);

  var sum = 0;

  var fuel = Math.floor(mass / 3) - 2;

  while (fuel >= 0) {
    sum += fuel;
    fuel = Math.floor(fuel / 3) - 2;
  }

  output += sum;

  console.log("Added Fuel: " + sum);
  console.log("New Fuel: " + output);
});
console.log("-----------------------");
console.log("[Day 1] Part 2: The second element's value is" + output);
