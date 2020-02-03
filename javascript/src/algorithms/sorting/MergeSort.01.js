function MergeSort(array, left, right) {
  if (right <= left) { return; }
  const mid = Math.floor((left + right) / 2);
  MergeSort(array, left, mid);
  MergeSort(array, mid+1, right);
  merge(array, left, mid, right);
}

function merge(array, left, mid, right) {
  const temp = new Array(right - left + 1);
  [i, j, k] = [left, mid+1, 0]
  while (i <= mid && j <= right) {
    temp[k++] = array[i] <= array[j] ? array[i++] : array[j++];
  }
  while (i <= mid) {
    temp[k++] = array[i++];
  }
  while (j <= right) {
    temp[k++] = array[j++];
  }
  for (let p = 0; p < temp.length; p += 1) {
    array[left + p] = temp[p];
  }
}

let array = [2,1,3,5,4];
MergeSort(array, 0, 4);
console.log(array);
