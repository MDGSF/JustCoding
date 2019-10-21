/**
 * @param {string[]} folder
 * @return {string[]}
 */
var removeSubfolders = function(folder) {
  folder.sort();

  p1 = 0;
  p2 = 1;
  result = [];
  result[0] = folder[0];
  while (true) {
    if (p2 >= folder.length) {
      break;
    }
    if (folder[p2].startsWith(folder[p1])) {
      p2++;
      continue;
    } else {
      result.push(folder[p2]);
      p1 = p2;
      p2++;
    }
  }
  return result;
};
