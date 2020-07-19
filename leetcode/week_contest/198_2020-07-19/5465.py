from typing import List


class Solution:

  def countSubTrees(self, n: int, edges: List[List[int]],
                    labels: str) -> List[int]:
    ans = [1] * n

    counter = {}
    for edge in edges:
      src, dest = edge[0], edge[1]
      if src in counter:
        counter[src] += 1
      else:
        counter[src] = 1
      if dest in counter:
        counter[dest] += 1
      else:
        counter[dest] = 1

    leaf = set()
    for k, v in counter.items():
      if v == 1 and k != 0:
        leaf.add(k)

    s = {}
    for edge in edges:
      src, dest = edge[0], edge[1]
      if src in leaf or dest == 0:
        src, dest = dest, src
        if src in s:
          s[src].append(dest)
        else:
          s[src] = [dest]
      else:
        if src in s:
          s[src].append(dest)
        else:
          s[src] = [dest]
        if dest in s:
          s[dest].append(src)
        else:
          s[dest] = [src]

    #print(s)

    visited = set()
    def dfs(node):
      visited.add(node)
      if node in s:
        subs = s[node]
        curm = {labels[node]: 1}
        for subnode in subs:
          if subnode in visited:
            continue
          subm = dfs(subnode)
          for k, v in subm.items():
            if k in curm:
              curm[k] += subm[k]
            else:
              curm[k] = v
        ans[node] = curm[labels[node]]
        return curm
      else:
        ans[node] = 1
        return {labels[node]: 1}

    dfs(0)

    return ans


n = 7
edges = [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]]
labels = "abaedcd"

n = 4
edges = [[0, 1], [1, 2], [0, 3]]
labels = "bbbb"

n = 5
edges = [[0, 1], [0, 2], [1, 3], [0, 4]]
labels = "aabab"

n = 6
edges = [[0, 1], [0, 2], [1, 3], [3, 4], [4, 5]]
labels = "cbabaa"

n = 7
edges = [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 6]]
labels = "aaabaaa"

n = 4
edges = [[0, 2], [0, 3], [1, 2]]
labels = "aeed"

n = 25
edges = [[4, 0], [5, 4], [12, 5], [3, 12], [18, 3], [10, 18], [8, 5], [16, 8],
         [14, 16], [13, 16], [9, 13], [22, 9], [2, 5], [6, 2], [1, 6], [11, 1],
         [15, 11], [20, 11], [7, 20], [19, 1], [17, 19], [23, 19], [24, 2],
         [21, 24]]
labels = "hcheiavadwjctaortvpsflssg"

s = Solution()
result = s.countSubTrees(n, edges, labels)
print(result)
