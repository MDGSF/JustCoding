function MergeSort(originalArray) {
  if (originalArray.length <= 1) {
    return originalArray;
  }

  const middleIndex = Math.floor(originalArray.length / 2);
  const leftArray = originalArray.slice(0, middleIndex);
  const rightArray = originalArray.slice(middleIndex, originalArray.length);

  const leftSortedArray = MergeSort(leftArray);
  const rightSortedArray = MergeSort(rightArray);

  return mergeSortedArrays(leftSortedArray, rightSortedArray);
}

function mergeSortedArrays(leftArray, rightArray) {
  let sortedArray = [];

  while (leftArray.length > 0 && rightArray.length > 0) {
    let minimumElement = null;

    if (leftArray[0] < rightArray[0]) {
      minimumElement = leftArray.shift();
    } else {
      minimumElement = rightArray.shift();
    }

    sortedArray.push(minimumElement);
  }

  if (leftArray.length > 0) {
    sortedArray = sortedArray.concat(leftArray);
  }

  if (rightArray.length > 0) {
    sortedArray = sortedArray.concat(rightArray);
  }

  return sortedArray;
}
