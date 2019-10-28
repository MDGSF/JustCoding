function BubbleSort(a) {
  for (let i = 1; i < a.length; i += 1) {
    for (let j = 0; j < a.length - i; j += 1) {
      if (a[j] > a[j + 1]) {
        [a[j], a[j + 1]] = [a[j + 1], a[j]];
      }
    }
  }
  return a;
}

module.exports = BubbleSort;
