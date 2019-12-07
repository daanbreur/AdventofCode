const readline = require("readline");
const fs = require("fs");
let output = 0;

const rl = readline.createInterface({
  input: fs.createReadStream("input.txt")
});

rl.on("line", mass => {
  console.log("Mass from file: ", mass);

  var sum = 0;

  var thisMass = mass;
  var thisMassFuel = Math.floor(thisMass / 3) - 2;

  while (thisMassFuel >= 0) {
    sum += thisMassFuel;
    thisMassFuel = Math.floor(thisMassFuel / 3) - 2;
  }

  output += sum;

  console.log("Added Fuel: " + sum);
  console.log("New Fuel: " + output);
});

console.log("[Day 1] Part 2: The second element's value is" + output);
