console.log("Hello via Bun to adventofcode23.com/2023/day/2");
const log = false;

const input = Bun.file("input.txt");
const inputText = await input.text();

const maxCubes = {
  red: 12,
  green: 13,
  blue: 14,
} as { [key: string]: number };

const dynMaxCubes = {
  red: 0,
  green: 0,
  blue: 0,
} as { [key: string]: number };

let sumPartTwo = 0;
const sumPartOne = inputText.split("\n").reduce((acc, line) => {
  if (!line.length) {
    return acc;
  }
  const [gamePart, contentPart] = line.split(":");
  const gameNum = parseInt(gamePart.split(" ")[1]);
  log && console.log(`${gameNum}: ${contentPart}`);

  let gamePossible = true;
  let dynMaxCubesForLine = Object.assign({}, dynMaxCubes);
  for (const setOfCubes of contentPart
    .trim()
    .replaceAll(";", ",")
    .split(", ")) {
    const [numS, color] = setOfCubes.split(" ").map((s) => s.trim());
    const num = parseInt(numS);
    log && console.log(`${setOfCubes}: >${num}< >${color}<`);

    if (num > (maxCubes[color] || Number.MAX_SAFE_INTEGER)) {
      gamePossible = false;
    }
    if (num > dynMaxCubesForLine[color]) {
      dynMaxCubesForLine[color] = num;
    }
  }
  sumPartTwo +=
    dynMaxCubesForLine.red * dynMaxCubesForLine.green * dynMaxCubesForLine.blue;
  return acc + (gamePossible ? gameNum : 0);
}, 0);
console.log(`partOne answer:\n${sumPartOne}`);
console.log(`partTwo answer:\n${sumPartTwo}`);
