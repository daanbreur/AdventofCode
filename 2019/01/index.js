const readline = require("readline");
const fs = require("fs");
const rl = readline.createInterface({
  input: fs.createReadStream("input.txt")
});

let output = 0;
let output2 = 0;

module.exports = main;

function main() {
  rl.on("line", mass => {
    console.log("Mass from file: ", mass);

    //part1
    output += Math.floor(mass / 3) - 2;
    console.log("[Day 1] Part 1 New Fuel : " + output);

    //part2
    var sum = 0;
    var fuel = Math.floor(mass / 3) - 2;

    while (fuel >= 0) {
      sum += fuel;
      fuel = Math.floor(fuel / 3) - 2;
    }

    output2 += sum;

    console.log("[Day 1] Part 2 Added Fuel: " + sum);
    console.log("[Day 1] Part 2 New Fuel : " + output2);
  });

  console.log("-----------------------");
  console.log("[Day 1] Part 1: The first element's value is" + output);
  console.log("[Day 1] Part 2: The second element's value is" + output2);
  console.log("-----------------------");
}
