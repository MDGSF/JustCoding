class Solution:
  def distributeCandies(self, candies: int, num_people: int) -> List[int]:
    result = [0] * num_people
    i, curCandy = 0, 1
    while candies > 0:
      if candies > curCandy:
        result[i] += curCandy
        candies -= curCandy
        curCandy += 1
        i = (i + 1) % num_people
      else:
        result[i] += candies
        candies = 0
    return result
