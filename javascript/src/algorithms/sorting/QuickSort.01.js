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
  for (let i = start; i < end; i += 1) {
    if (a[i] < pivot) {
      swap(a, i, partitionIndex);
      partitionIndex += 1;
    }
  }
  swap(a, partitionIndex, end);
  return partitionIndex;
}

function swap(a, i, j) {
  [a[i], a[j]] = [a[j], a[i]];
}
