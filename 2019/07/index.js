const fs = require("fs");
module.exports = main;

function main() {
  fs.readFile("./input.txt", "utf8", (err, data) => {
    let arrayA = data.split(","),
      arrayB = data.split(","),
      arrayC = data.split(","),
      arrayD = data.split(","),
      arrayE = data.split(",");

    let phases = [0, 1, 2, 3, 4];

    let randomA = 0,
      randomB = 0,
      randomC = 0,
      randomD = 0,
      randomE = 0;

    const firstPart = parseIntCode(
      arrayE,
      randomE,
      parseIntCode(
        arrayD,
        randomD,
        parseIntCode(
          arrayC,
          randomC,
          parseIntCode(arrayB, randomB, parseIntCode(arrayA, randomA, "0"))
        )
      )
    );
    // const secondPart = parseIntCode(array, "5");
    console.log(`[Day 5] Part 1: The first element's value is ${firstPart}`);
    // console.log(`[Day 5] Part 2: The second element's value is ${secondPart}`);
  });
}

function parseIntCode(array, inputPhase, input) {
  let firstTime = true;
  for (let i = 0; i < array.length; i++) {
    let opcode = array[i];
    while (opcode.split("").length < 5) opcode = `0${opcode}`;
    const parameterMode = opcode.split(""),
      firstParameter = parseInt(array[i + 1]),
      secondParameter = parseInt(array[i + 2]),
      overrideParameter = parseInt(array[i + 3]);
    let done = false;
    switch (parseInt(opcode.substr(opcode.length - 2, opcode.length - 1))) {
      case 1:
        array[overrideParameter] = (
          (parameterMode[2] === "0"
            ? parseInt(array[firstParameter])
            : firstParameter) +
          (parameterMode[1] === "0"
            ? parseInt(array[secondParameter])
            : secondParameter)
        ).toString();
        i += 3;
        break;
      case 2:
        array[overrideParameter] = (
          (parameterMode[2] === "0"
            ? parseInt(array[firstParameter])
            : firstParameter) *
          (parameterMode[1] === "0"
            ? parseInt(array[secondParameter])
            : secondParameter)
        ).toString();
        i += 3;
        break;
      case 3:
        if (firstTime) {
          firstTime = false;
          array[firstParameter] = inputPhase;
        } else array[firstParameter] = input;
        i += 1;
        break;
      case 4:
        const returnValue =
          parameterMode[2] === "0"
            ? parseInt(array[firstParameter])
            : firstParameter;
        if (returnValue !== 0) return returnValue;
        i += 1;
        break;
      case 5:
        if (
          (parameterMode[2] === "0"
            ? parseInt(array[firstParameter])
            : firstParameter) !== 0
        )
          i =
            (parameterMode[1] === "0"
              ? parseInt(array[secondParameter])
              : secondParameter) - 1;
        else i += 2;
        break;
      case 6:
        if (
          (parameterMode[2] === "0"
            ? parseInt(array[firstParameter])
            : firstParameter) === 0
        )
          i =
            (parameterMode[1] === "0"
              ? parseInt(array[secondParameter])
              : secondParameter) - 1;
        else i += 2;
        break;
      case 7:
        if (
          (parameterMode[2] === "0"
            ? parseInt(array[firstParameter])
            : firstParameter) <
          (parameterMode[1] === "0"
            ? parseInt(array[secondParameter])
            : secondParameter)
        )
          array[overrideParameter] = "1";
        else array[overrideParameter] = "0";
        i += 3;
        break;
      case 8:
        if (
          (parameterMode[2] === "0"
            ? parseInt(array[firstParameter])
            : firstParameter) ==
          (parameterMode[1] === "0"
            ? parseInt(array[secondParameter])
            : secondParameter)
        )
          array[overrideParameter] = "1";
        else array[overrideParameter] = "0";
        i += 3;
        break;
      case 99:
        done = true;
        break;
    }
    if (done) return array[0];
  }
}
