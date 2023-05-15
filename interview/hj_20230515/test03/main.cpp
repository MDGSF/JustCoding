#include <algorithm>
#include <iostream>
#include <vector>

int calc_balls_num(const std::vector<int>& bucket_balls_nums, int cur_capacity) {
  int cur_all_balls_num = 0;
  int N = bucket_balls_nums.size();
  for (int i = 0; i < N; ++i) {
    if (bucket_balls_nums[i] > cur_capacity) {
      cur_all_balls_num += cur_capacity;
    } else {
      cur_all_balls_num += bucket_balls_nums[i];
    }
  }
  return cur_all_balls_num;
}

int main() {
  int SUM;
  int N; // 数组的长度
  std::cin >> SUM >> N;

  int max_balls_num = 0; // 小球最多的那个桶的小球数量
  int all_balls_num = 0; // 所有小球总数
  std::vector<int> bucket_balls_nums(N, 0);
  for (int i = 0; i < N; ++i) {
    int cur_balls_num; // 当前桶的小球数量
    std::cin >> cur_balls_num;

    max_balls_num = std::max(max_balls_num, cur_balls_num);
    all_balls_num += cur_balls_num;
    bucket_balls_nums[i] = cur_balls_num;
  }

  if (all_balls_num <= SUM) {
    std::cout << "[]" << std::endl;
    return 0;
  }

  //std::cout << "SUM: " << SUM << std::endl;
  //std::cout << "N: " << N << std::endl;
  //std::cout << "max_balls_num: " << max_balls_num << std::endl;
  //std::cout << "all_balls_num: " << all_balls_num << std::endl;
  //std::cout << "bucket_balls_nums: ";
  //for (int i = 0; i < N; ++i) {
  //  std::cout << bucket_balls_nums[i] << " ";
  //}
  //std::cout << std::endl;

  int capacity_wanted = 0;
  int left = 0;
  int right = max_balls_num;
  while (left < right) {
    int mid = left + (right - left) / 2;
    //std::cout << "left:" << left << ", right:" << right << ", mid:" << mid << std::endl;

    int cur_capacity = mid;
    int next_capacity = cur_capacity + 1;
    int cur_all_balls_num = calc_balls_num(bucket_balls_nums, cur_capacity);
    int next_all_balls_num = calc_balls_num(bucket_balls_nums, next_capacity);

    if (cur_all_balls_num <= SUM && next_all_balls_num > SUM) {
      capacity_wanted = cur_capacity;
      break;
    } else if (cur_all_balls_num <= SUM && next_all_balls_num <= SUM) {
      left = next_capacity;
    } else {
      right = cur_capacity - 1;
    }
  }

  //std::cout << "capacity_wanted: " << capacity_wanted << std::endl;

  std::vector<int> take_balls_num(N, 0);
  for (int i = 0; i < N; ++i) {
    if (bucket_balls_nums[i] > capacity_wanted) {
      take_balls_num[i] = bucket_balls_nums[i] - capacity_wanted;
    }
  }

  // [0,1,0,3,3,0,2]
  std::cout << "[";
  for (int i = 0; i < N; ++i) {
    if (i != 0) {
      std::cout << ",";
    }
    std::cout << take_balls_num[i];
  }
  std::cout << "]" << std::endl;

  return 0;
}
