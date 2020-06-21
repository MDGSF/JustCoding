from typing import List

class Solution:
  def getFolderNames(self, names: List[str]) -> List[str]:
    s = set()
    result = []
    for name in names:
      if name in s:
        for i in range(1, 50001):
          newname = name + "(" + str(i) + ")"
          if newname in s:
            continue
          else:
            s.add(newname)
            result.append(newname)
            break
      else:
        s.add(name)
        result.append(name)
    return result

s = Solution()

# 输入：names = ["pes","fifa","gta","pes(2019)"]
# 输出：["pes","fifa","gta","pes(2019)"]
# names = ["pes","fifa","gta","pes(2019)"]
# result = s.getFolderNames(names)
# print(result)


# 输入：names = ["gta","gta(1)","gta","avalon"]
# 输出：["gta","gta(1)","gta(2)","avalon"]
# names = ["gta","gta(1)","gta","avalon"]
# result = s.getFolderNames(names)
# print(result)


# 输入：names = ["onepiece","onepiece(1)","onepiece(2)","onepiece(3)","onepiece"]
# 输出：["onepiece","onepiece(1)","onepiece(2)","onepiece(3)","onepiece(4)"]
names = ["onepiece","onepiece(1)","onepiece(2)","onepiece(3)","onepiece"]
result = s.getFolderNames(names)
print(result)
