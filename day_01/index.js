const {readFileSync} = require("fs");

const masses =
  readFileSync("./input")
    .toString("utf-8")
    .split("\n")
    .filter(x => x.length > 0)
    .map(x => parseInt(x));

function calcFuel(mass) {
  return Math.floor(mass / 3) - 2;
}

const fuels = masses.map(mass => {
  let sum = 0;
  let additionalFuel = calcFuel(mass);
  do {
    sum += additionalFuel;
    additionalFuel = calcFuel(additionalFuel);
  } while (additionalFuel > 0);
  return sum;
});

console.log(fuels.reduce(((a, b) => a + b), 0));