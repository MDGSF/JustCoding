function InsertionSort(a) {
  for (let i = 0; i < a.length; i += 1) {
    let currentIndex = i;
    while (
      a[currentIndex - 1] !== undefined &&
      a[currentIndex - 1] > a[currentIndex]
    ) {
      [a[currentIndex - 1], a[currentIndex]] = [
        a[currentIndex],
        a[currentIndex - 1]
      ];
      currentIndex -= 1;
    }
  }
}

module.exports = InsertionSort;
