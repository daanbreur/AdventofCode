module.exports = main;

function main(data) {
  let array = data.split(",");
  const firstPart = parseIntCode(array, "1");
  array = data.split(",");
  const secondPart = parseIntCode(array, "5");
  return [
    {
      message: "The diagnostic code without the 0s is",
      value: firstPart
    },
    { message: "The diagnostic code for system ID 5 is", value: secondPart }
  ];
}

function parseIntCode(array, input) {
  for (let i = 0; i < array.length; i++) {
    let opcode = array[i];
    while (opcode.split("").length < 5) opcode = `0${opcode}`;
    const parameterMode = opcode.split("");
    const firstParameter = parseInt(array[i + 1]);
    const secondParameter = parseInt(array[i + 2]);
    const overrideParameter = parseInt(array[i + 3]);
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
        array[firstParameter] = input;
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
            : firstParameter) ===
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
