#include <iostream>
#include <vector>
#include <map>
#include <set>

int main() {
  int n;
  std::cin >> n;
  std::vector<int> array(n, 0);
  for (int i = 0; i < n; ++i) {
    std::cin >> array[i];
  }

  std::map<int, int> counter;
  for (int i = 0; i < array.size(); ++i) {
    counter[array[i]] += 1;
  }

  int largest_value = 0;
  for (auto& it : counter) {
    if (it.second >= largest_value) {
      largest_value = it.second;
    }
  }

  std::set<int> largest_nums;
  for (auto& it : counter) {
    if (it.second == largest_value) {
      largest_nums.insert(it.first);
    }
  }

  int min_length = 999999;
  for (int num : largest_nums) {
    // std::cout << num << std::endl;
    int left = 0;
    int right = array.size() - 1;
    while (left < array.size()) {
      if (array[left] == num) {
        break;
      } else {
        left += 1;
      }
    }
    while (right >= 0) {
      if (array[right] == num) {
        break;
      } else {
        right -= 1;
      }
    }
    int cur_length = right - left + 1;
    if (cur_length < min_length) {
      min_length = cur_length;
    }
  }

  std::cout << min_length << std::endl;

  return 0;
}
