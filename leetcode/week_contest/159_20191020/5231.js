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

    const s1 = folder[p1];
    const s2 = folder[p2];
    if (s2.startsWith(s1) && s2.split('/').length !== s1.split('/').length) {
      p2++;
      continue;
    } else {
      result.push(s2);
      p1 = p2;
      p2++;
    }
  }
  return result;
};
