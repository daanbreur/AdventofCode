module.exports = main;

let input = "357253-892942";

let start = input.split("-")[0],
  end = input.split("-")[1],
  count = start,
  amountPasswords = 0,
  amountPasswordsPart2 = 0;

function main() {
  while (count <= end) {
    let prevNumber = 0,
      doubleValue = 0,
      beforePrevNumber = 0;
    let skip = false,
      notSkip = false,
      hasDouble = false;

    for (const number of count.toString().split("")) {
      if (parseInt(number) < prevNumber) {
        skip = true;
        break;
      }

      if (parseInt(number) === prevNumber) {
        notSkip = true;
        if (!doubleValue || doubleValue === prevNumber) {
          if (beforePrevNumber) hasDouble = prevNumber !== beforePrevNumber;
          else hasDouble = true;

          if (hasDouble) doubleValue = prevNumber;
          else doubleValue = undefined;
        }
      }

      beforePrevNumber = prevNumber;
      prevNumber = parseInt(number);
    }
    count++;
    if (skip || !notSkip) continue;
    amountPasswords++;
    if (doubleValue) amountPasswordsPart2++;
  }

  console.log(`[Day 4 Part 1] Answer is: ${amountPasswords}`);
  console.log(`[Day 4 Part 2] Answer is: ${amountPasswordsPart2}`);
}
