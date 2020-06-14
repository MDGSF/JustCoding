import bisect

class Solution:
  def findBestValue(self, arr: List[int], target: int) -> int:
    arr.sort()
    n = len(arr)
    prefix = [0]
    for num in arr:
      prefix.append(prefix[-1] + num)

    l, r, ans = 0, max(arr), -1
    while l <= r:
      mid = (l + r) // 2
      i = bisect.bisect_left(arr, mid)
      cur = prefix[i] + (n - i) * mid
      if cur <= target:
        ans = mid
        l = mid + 1
      else:
        r = mid - 1

    def check(x):
      return sum(x if num >= x else num for num in arr)

    small = check(ans)
    big = check(ans + 1)
    return ans if abs(small - target) <= abs(big - target) else ans + 1
