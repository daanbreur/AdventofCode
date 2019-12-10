let fs = require("fs");
let data = fs
  .readFileSync("./input.txt")
  .toString()
  .split("\n");

module.exports = main;

function getWirePoints(wireMovements) {
  let wirePoints = new Set();
  let x = 0,
    y = 0;
  wireMovements.forEach(movement => {
    let direction = movement.substring(0, 1);
    let spaces = parseInt(movement.substring(1, movement.length));

    for (let i = 0; i < spaces; i++) {
      switch (direction) {
        case "U":
          y--;
          break;
        case "D":
          y++;
          break;
        case "L":
          x--;
          break;
        case "R":
          x++;
          break;
      }

      if (!wirePoints.has(`${x},${y}`)) {
        wirePoints.add(`${x},${y}`);
      }
    }
  });

  return wirePoints;
}

function getCrossOverPoints(wireAPoints, wireBPoints) {
  let crossOverLocations = [];

  wireAPoints.forEach(point => {
    if (wireBPoints.has(point)) {
      crossOverLocations.push(point);
    }
  });

  return crossOverLocations;
}

function findMinimumDistance(crossOverLocations) {
  let minimumDistance = findManhattanDistanceToStart(crossOverLocations[0]);

  crossOverLocations.forEach(location => {
    let curDistance = findManhattanDistanceToStart(location);
    if (curDistance < minimumDistance) {
      minimumDistance = curDistance;
    }
  });

  return minimumDistance;
}

function findManhattanDistanceToStart(location) {
  let x = parseInt(location.split(",")[0]);
  let y = parseInt(location.split(",")[1]);
  return Math.abs(x) + Math.abs(y);
}

function main() {
  console.log(
    `[Day 3 Part 1] Answer is: ${findMinimumDistance(
      getCrossOverPoints(
        getWirePoints(data[0].split(",")),
        getWirePoints(data[1].split(","))
      )
    )}`
  );
}
