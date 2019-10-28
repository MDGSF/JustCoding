function SelectionSort(a) {
  for (let i = 0; i < a.length - 1; i += 1) {
    let minIndex = i;

    for (let j = i + 1; j < a.length; j += 1) {
      if (a[j] < a[minIndex]) {
        minIndex = j;
      }
    }

    if (minIndex !== i) {
      [a[minIndex], a[i]] = [a[i], a[minIndex]];
    }
  }
  return a;
}

module.exports = SelectionSort;
