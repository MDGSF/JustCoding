class Solution:
  def insertBits(self, N: int, M: int, i: int, j: int) -> int:
    mi, ni = 0, i
    while ni <= j:
      mbit = ((M & (1 << mi)) >> mi)
      if mbit == 0:
        N = N & (~(1 << ni))
      else:
        N = N | (1 << ni)
      mi += 1
      ni += 1
    return N

N = 2032243561
M = 10
i = 24
j = 29
print('N =', bin(N))
print('M =', bin(M))
s = Solution()
result = s.insertBits(N, M, i, j)
print(result, bin(result))

# 0b1 111 001 001 000 011 001 001 101 101 001
# 0b1     010 001 000 011 001 001 101 101 001
