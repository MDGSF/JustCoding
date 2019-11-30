# 二分查找模板

```
left, right = 0, array.length - 1
while (left <= right) {
    mid = (left + right) / 2;
    if (array[mid] === target) {
        return
    } else if (array[mid] < target) {
        left = mid + 1;
    } else {
        right = mid - 1;
    }
}
```
