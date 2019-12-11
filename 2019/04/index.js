module.exports = main;

let start = 357253,
  end = 892942,
  count = start,
  amountPasswords = 0,
  amountPasswordsPart2 = 0;

function main() {
  while (count <= end) {
    let prevNumber = 0;
    let skip = false,
      notSkip = false;

    for (const number of count.toString().split("")) {
      if (parseInt(number) < prevNumber) {
        skip = true;
        break;
      }

      if (parseInt(number) === prevNumber) {
        notSkip = true;
      }

      prevNumber = parseInt(number);
    }
    count++;
    if (skip || !notSkip) continue;
    amountPasswords++;
  }

  console.log(`[Day 4 Part 1] Answer is: ${amountPasswords}`);
  console.log(`[Day 4 Part 2] Answer is: ${amountPasswordsPart2}`);
}
