function ShellSort(a) {
  for (let gap = Math.floor(a.length / 2); gap > 0; gap = Math.floor(gap / 2)) {
    for (let i = 0; i < gap; i += 1) {
      InsertionSortWithGap(a, gap, i);
    }
  }
  return a;
}

function InsertionSortWithGap(a, gap, start) {
  for (let i = start; i < a.length; i += gap) {
    for (
      let currentIndex = i;
      a[currentIndex - gap] !== undefined &&
      a[currentIndex - gap] > a[currentIndex];
      currentIndex -= gap
    ) {
      const temp = a[currentIndex - gap];
      a[currentIndex - gap] = a[currentIndex];
      a[currentIndex] = temp;
    }
  }
}

module.exports = ShellSort;
