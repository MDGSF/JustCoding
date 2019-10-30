function partition(a, start, end) {
  const pivot = a[end];
  let partitionIndex = start;
  for (let i = start; i < end; i += 1) {
    if (a[i] > pivot) {
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

function topk(a, k) {
  if (k > a.length || k <= 0) {
    return null;
  }

  k -= 1;
  let start = 0;
  let end = a.length - 1;
  let partitionIndex = null;
  while (partitionIndex !== k) {
    partitionIndex = partition(a, start, end);
    if (partitionIndex < k) {
      start = partitionIndex + 1;
    } else {
      end = partitionIndex - 1;
    }
  }
  return a[k];
}

module.exports = topk;
