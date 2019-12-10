let fs = require("fs");
let data = fs
  .readFileSync("./input.txt")
  .toString()
  .split("\n");

module.exports = main;

function getWirePoints(wireMovements) {
  let wirePoints = new Map();
  let x = 0,
    y = 0,
    distance = 1;

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
        wirePoints.set(`${x},${y}`, distance);
      }
      distance++;
    }
  });

  return wirePoints;
}

function getCrossOverPoints(wireAPoints, wireBPoints) {
  let crossOverPoints = [];

  wireAPoints.forEach((value, key) => {
    if (wireBPoints.has(key)) {
      crossOverPoints.push(key);
    }
  });

  return crossOverPoints;
}

function findMinimumTotalDistanceTravelled(
  crossOverLocations,
  wireOne,
  wireTwo
) {
  let minKey = crossOverLocations[0];
  let minimumTravelDistance = wireOne.get(minKey) + wireTwo.get(minKey);

  crossOverLocations.forEach(key => {
    let totalDistance = wireOne.get(key) + wireTwo.get(key);

    if (totalDistance < minimumTravelDistance) {
      minimumTravelDistance = totalDistance;
      minKey = key;
    }
  });

  return { minimumTravelDistance, minKey };
}

function main() {
  console.log(
    `[Day 3 Part 2] Answer is: ${
      findMinimumTotalDistanceTravelled(
        getCrossOverPoints(
          getWirePoints(data[0].split(",")),
          getWirePoints(data[1].split(","))
        ),
        getWirePoints(data[0].split(",")),
        getWirePoints(data[1].split(","))
      ).minimumTravelDistance
    }\n----- OBJECT -----`
  );
  console.log(
    findMinimumTotalDistanceTravelled(
      getCrossOverPoints(
        getWirePoints(data[0].split(",")),
        getWirePoints(data[1].split(","))
      ),
      getWirePoints(data[0].split(",")),
      getWirePoints(data[1].split(","))
    )
  );
}
