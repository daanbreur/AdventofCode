const width = 25,
  heigth = 6,
  pixelsLayer = width * heigth;

const fs = require("fs");

module.exports = main;

function main() {
  fs.readFile("./input.txt", "utf8", (err, data) => {
    const layers = data.match(new RegExp(".{1," + pixelsLayer + "}", "g"));
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

    decodeImage(layers);

    console.log(
      `[2019 Day 8 Part 1] The First Elements value is ${numbersInLayer.one *
        numbersInLayer.two}`
    );
    console.log(`[2019 Day 8 Part 2] The Second Elements value is `);
  });
}

function decodeImage(layers) {
  for (const layer of layers) {
    const rows = layer.match(new RegExp(".{1," + width + "}", "g"));
  }
}
