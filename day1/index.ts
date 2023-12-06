console.log("Hello via Bun to adventofcode23.com/2023/day/1");

const literalDigits = [
  "zero",
  "one",
  "two",
  "three",
  "four",
  "five",
  "six",
  "seven",
  "eight",
  "nine",
];

function getLiteralDigitOrNull(candidate: string, fromStart: boolean = true) {
  for (let i = 0; i < literalDigits.length; i++) {
    const digitCandidate = literalDigits[i];
    if (
      (fromStart && candidate.startsWith(digitCandidate)) ||
      (!fromStart && candidate.endsWith(digitCandidate))
    ) {
      return i;
    }
  }
  return null;
}

function getDigitOrNull(candidate: string) {
  const parsed = parseInt(candidate);
  if (parsed.toString() == candidate) {
    return parsed;
  }
  return null;
}

const input = Bun.file("input.txt");
const inputText = await input.text();

const lines = inputText.split("\n");

const log = false;
function getSum(partTwo: boolean = false) {
  let sum = 0;
  for (const line of lines) {
    if (!line.length) {
      continue;
    }
    let leftDigit = null;
    let rightDigit = null;
    let leftCursor = 0;
    let rightCursor = line.length - 1;
    log && console.log(line);
    while (
      leftCursor <= rightCursor &&
      (leftDigit == null || rightDigit == null)
    ) {
      if (leftDigit == null) {
        leftDigit = getDigitOrNull(line[leftCursor]);
        const lc = line.substring(leftCursor, leftCursor + 5);
        if (partTwo && leftDigit == null) {
          leftDigit = getLiteralDigitOrNull(lc);
        }
        log && console.log("l", lc, leftDigit);
        if (leftDigit == null) {
          leftCursor++;
        }
      }
      if (rightDigit == null) {
        rightDigit = getDigitOrNull(line[rightCursor]);
        const lc = line.substring(rightCursor - 4, rightCursor + 1);
        if (partTwo && rightDigit == null) {
          rightDigit = getLiteralDigitOrNull(lc, false);
        }
        log && console.log("r", lc, rightDigit);
        if (rightDigit == null) {
          rightCursor--;
        }
      }
    }
    if (rightDigit == null) {
      rightDigit = leftDigit;
    }
    if (leftDigit == null) {
      leftDigit = rightDigit;
    }
    if (leftDigit == null) {
      //throw new Error("no digit on line " + line);
      continue;
    }
    log && console.log(`${leftDigit}${rightDigit}`);
    sum += parseInt(`${leftDigit}${rightDigit}`);
  }
  console.log(`${partTwo ? "partTwo" : "partOne"} answer:\n${sum}`);
}
getSum(false);
getSum(true);
