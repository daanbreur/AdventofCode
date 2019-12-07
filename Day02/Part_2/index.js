const fs = require("fs");

fs.readFile("./input.txt", "utf8", (err, data) => {
  array = data.split(",");

  let firstElementCounter = 0;
  let secondElementCounter = 0;
  while (firstElementCounter <= 99 && secondElementCounter <= 99) {
    array[1] = firstElementCounter.toString();
    array[2] = secondElementCounter.toString();
    if (parseInt(parseIntCode(array)) === 19690720)
      console.log(
        `[Day 2] Part 2: Calculated answer (100 * noun + verb): ${100 *
          firstElementCounter +
          secondElementCounter}`
      );

    array = data.split(",");
    if (firstElementCounter < 99) firstElementCounter++;
    else {
      firstElementCounter = 0;
      secondElementCounter++;
    }
  }
});

function parseIntCode(array) {
  let count = 0;
  for (const opcode of array) {
    if (count % 4 === 0) {
      const firstElement = parseInt(array[count + 1]),
        secondElement = parseInt(array[count + 2]),
        overrideElement = parseInt(array[count + 3]);
      let done = false;
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
