const width = 25;
const heigth = 6;

const fs = require("fs");

module.exports = main;

function main() {
  fs.readFile("./input.txt", "utf8", (err, data) => {
    const layers = data.match(/.{1,150}/g);
    let zeroCount = { count: Infinity, layer: undefined };
    for (const layer of layers) {
      const zerosInLayer = (layer.match(/0/g) || []).length;
      if (zeroCount.count > zerosInLayer)
        zeroCount = { count: zerosInLayer, layer: layer };
    }

    const numbersInLayer = {
      one: (zeroCount.layer.match(/1/g) || []).length,
      two: (zeroCount.layer.match(/2/g) || []).length
    };

    console.log(
      `[2019 Day 8 Part 1] The First Elements value is ${numbersInLayer.one *
        numbersInLayer.two}`
    );
    console.log(``);
  });
}
