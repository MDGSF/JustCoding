const ZERO_OR_MORE_CHARS = "*";
const ANY_CHAR = ".";

function regularExpressionMatching(string, pattern) {
  const matchMatrix = Array(string.length + 1)
    .fill(null)
    .map(() => {
      return Array(pattern.length + 1).fill(null);
    });

  matchMatrix[0][0] = true;

  for (let columnIndex = 1; columnIndex <= pattern.length; columnIndex += 1) {
    const patternIndex = columnIndex - 1;

    if (pattern[patternIndex] === ZERO_OR_MORE_CHARS) {
      matchMatrix[0][columnIndex] = matchMatrix[0][columnIndex - 2];
    } else {
      matchMatrix[0][columnIndex] = false;
    }
  }

  for (let rowIndex = 1; rowIndex <= string.length; rowIndex += 1) {
    matchMatrix[rowIndex][0] = false;
  }

  for (let rowIndex = 1; rowIndex <= string.length; rowIndex += 1) {
    for (let columnIndex = 1; columnIndex <= pattern.length; columnIndex += 1) {
      const stringIndex = rowIndex - 1;
      const patternIndex = columnIndex - 1;

      if (pattern[patternIndex] === ZERO_OR_MORE_CHARS) {
        if (matchMatrix[rowIndex][columnIndex - 2] === true) {
          matchMatrix[rowIndex][columnIndex] = true;
        } else if (
          (pattern[patternIndex - 1] === string[stringIndex] ||
            pattern[patternIndex - 1] === ANY_CHAR) &&
          matchMatrix[rowIndex - 1][columnIndex] === true
        ) {
          matchMatrix[rowIndex][columnIndex] = true;
        } else {
          matchMatrix[rowIndex][columnIndex] = false;
        }
      } else if (
        pattern[patternIndex] === string[stringIndex] ||
        pattern[patternIndex] === ANY_CHAR
      ) {
        matchMatrix[rowIndex][columnIndex] =
          matchMatrix[rowIndex - 1][columnIndex - 1];
      } else {
        matchMatrix[rowIndex][columnIndex] = false;
      }
    }
  }

  return matchMatrix[string.length][pattern.length];
}

module.exports = regularExpressionMatching;
