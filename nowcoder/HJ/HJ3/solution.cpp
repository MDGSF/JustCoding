#include <iostream>
#include <algorithm>
#include <set>
#include <vector>

int main() {
  int n = 0;
  std::cin >> n;

  std::set<int> s;
  for (int i = 0; i < n; ++i) {
    int num = 0;
    std::cin >> num;
    s.insert(num);
  }

  std::vector<int> nums(s.begin(), s.end());
  std::sort(nums.begin(), nums.end());

  for (auto num : nums) {
    std::cout << num << std::endl;
  }

  return 0;
}
