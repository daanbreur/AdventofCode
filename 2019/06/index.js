const fs = require("fs");
let orbits;

module.exports = main;

function main() {
  fs.readFile("./input.txt", "utf8", (err, data) => {
    const orbitMap = data.split(/\r?\n/g);
    const results = {};

    for (const orbit of orbitMap) {
      const orbitSplit = orbit.split(")");
      if (results[orbitSplit[0]] === undefined) results[orbitSplit[0]] = [];
      results[orbitSplit[0]].push(orbitSplit[1]);
    }
    for (const key of Object.keys(results)) {
      let skip = false;
      for (const value of Object.values(results)) {
        if (value.includes(key)) {
          skip = true;
          break;
        }
      }
      if (skip) continue;
      const orbits = getOrbits(results, key, 0);
      console.log(orbits);
    }
  });
}

function getOrbits(results, key, i) {
  let count = 0;
  const valueArray = results[key];
  if (valueArray === undefined) return count;
  i++;
  for (const value of valueArray) {
    count += i;
    count += getOrbits(results, value, i);
  }
  return count;
}
