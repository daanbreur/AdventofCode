const fs = require("fs");
let done = false;

fs.readFile("./input.txt", "utf8", (err, data) => {
  if (err) {
    console.error(err);
    return;
  }

  let array = data.split(",");
  array[1] = "12";
  array[2] = "2";
  console.log(
    `[Day 2] Part 1: The first element's value is ${parseIntCode(array)}`
  );
});

function parseIntCode(array) {
  let count = 0;
  for (const opcode of array) {
    if (count % 4 === 0) {
      const firstElement = parseInt(array[count + 1]),
        secondElement = parseInt(array[count + 2]),
        overrideElement = parseInt(array[count + 3]);
      switch (parseInt(opcode)) {
        case 1:
          array[overrideElement] = (
            parseInt(array[firstElement]) + parseInt(array[secondElement])
          ).toString();
          break;
        case 2:
          array[overrideElement] = (
            parseInt(array[firstElement]) * parseInt(array[secondElement])
          ).toString();
          break;
        case 99:
          done = true;
          break;
      }
      if (done) return array[0];
    }
    count++;
  }
}
