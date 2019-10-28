function QuickSort(a, start = 0, end = a.length - 1) {
  if (start < end) {
    const partitionIndex = partition(a, start, end);
    QuickSort(a, start, partitionIndex - 1);
    QuickSort(a, partitionIndex + 1, end);
  }
  return a;
}

function partition(a, start, end) {
  const pivot = a[end];
  let partitionIndex = start;
  for (let currentIndex = start; currentIndex < end; currentIndex += 1) {
    if (a[currentIndex] < pivot) {
      [a[currentIndex], a[partitionIndex]] = [
        a[partitionIndex],
        a[currentIndex]
      ];
      partitionIndex += 1;
    }
  }

  [a[partitionIndex], a[end]] = [a[end], a[partitionIndex]];
  return partitionIndex;
}

module.exports = QuickSort;
