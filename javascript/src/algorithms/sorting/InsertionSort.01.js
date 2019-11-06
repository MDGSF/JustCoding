function InsertionSort(a) {
  for (let i = 1; i < a.length; i++) {
    for (let j = i; a[j - 1] && a[j - 1] > a[j]; j--) {
      [a[j - 1], a[j]] = [a[j], a[j - 1]];
    }
  }
}
