executeDay(2019, 2);

function executeDay(year, day) {
  year = year.toString();
  day = `${day <= 9 ? "0" : ""}${day.toString()}`;
  process.chdir(`./${year}/${day}`);
  require(`./${year}/${day}/index`)();
}
