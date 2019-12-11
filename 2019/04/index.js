module.exports = main;

function main() {
  let start = 357253;
  let end = 892942;
  let count = 357253;
  while (count <= end) {
    let prevNumber = 0;
    let skip = false;
    let correct = false
    for (const number of count.toString().split("")) {
      if (parseInt(number) < prevNumber) {
        skip = true
        break
      }
      prevNumber = number;
      console.log()
    }
    count++;
    if (skip || !correct) continue;
  }
}
