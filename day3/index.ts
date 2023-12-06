const log = true;
function sumPartNumbers(schematic: string[]): number {
  let sum = 0;
  const countedNumbers = new Set<string>();

  function isSymbol(char: string): boolean {
    return isNaN(parseInt(char)) && char != ".";
  }

  // Returns the full number and its starting position in the row
  function extractFullNumber(x: number, y: number): [string, number] {
    let numStr = "";
    let dy = y;

    // Go left to the start of the number
    while (dy >= 0 && !isNaN(parseInt(schematic[x][dy]))) {
      dy--;
    }
    dy++;

    let numStart = dy;

    // Extract the full number
    while (dy < schematic[x].length && !isNaN(parseInt(schematic[x][dy]))) {
      numStr += schematic[x][dy];
      dy++;
    }

    return [numStr, numStart];
  }

  for (let x = 0; x < schematic.length; x++) {
    for (let y = 0; y < schematic[x].length; y++) {
      const symbol = schematic[x][y];
      if (isSymbol(schematic[x][y])) {
        log && console.log(`Found symbol: ${schematic[x][y]} at (${x}, ${y})`);
        // Check all adjacent cells
        for (let i = -1; i <= 1; i++) {
          for (let j = -1; j <= 1; j++) {
            if (i === 0 && j === 0) continue;
            const nx = x + i;
            const ny = y + j;
            if (
              nx >= 0 &&
              nx < schematic.length &&
              ny >= 0 &&
              ny < schematic[nx].length
            ) {
              const char = schematic[nx][ny];
              if (!isNaN(parseInt(char))) {
                const [fullNumber, numStart] = extractFullNumber(nx, ny);
                const numKey = `${nx},${numStart}:${fullNumber}`;
                if (!countedNumbers.has(numKey)) {
                  countedNumbers.add(numKey);
                  sum += parseInt(fullNumber);
                  log &&
                    console.log(
                      `Found number: ${fullNumber} at (${nx}, ${numStart})`
                    );
                }
              }
            }
          }
        }
      }
    }
  }

  return sum;
}

// Example usage
const engineSchematic = [
  "467..114..",
  "...*......",
  "..35..633.",
  "......#...",
  "617*......",
  ".....+.58.",
  "..592.....",
  "......755.",
  "...$.*....",
  ".664.598..",
];

const input = Bun.file("input.txt");
const inputText = await input.text();
console.log(
  `Total sum of part numbers: ${sumPartNumbers(inputText.split("\n"))}`
);

console.log(`Total sum of part numbers: ${sumPartNumbers(engineSchematic)}`);
