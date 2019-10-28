function CountingSort(
  originalArray,
  smallestElement = undefined,
  biggestElement = undefined
) {
  let detectedSmallestElement = smallestElement || 0;
  let detectedBiggestElement = biggestElement || 0;

  if (smallestElement === undefined || biggestElement === undefined) {
    for (let i = 0; i < originalArray.length; i += 1) {
      if (originalArray[i] > detectedBiggestElement) {
        detectedBiggestElement = originalArray[i];
      }
      if (originalArray[i] < detectedSmallestElement) {
        detectedSmallestElement = originalArray[i];
      }
    }
  }

  const buckets = Array(
    detectedBiggestElement - detectedSmallestElement + 1
  ).fill(0);

  originalArray.forEach(element => {
    buckets[element - detectedSmallestElement] += 1;
  });

  for (let i = 1; i < buckets.length; i += 1) {
    buckets[i] += buckets[i - 1];
  }

  buckets.pop();
  buckets.unshift(0);

  const sortedArray = Array(originalArray.length).fill(null);
  for (let i = 0; i < originalArray.length; i += 1) {
    const element = originalArray[i];
    const elementSortedPosition = buckets[element - detectedSmallestElement];
    sortedArray[elementSortedPosition] = element;
    buckets[element - detectedSmallestElement] += 1;
  }
  return sortedArray;
}
