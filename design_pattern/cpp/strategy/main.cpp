#include <iostream>
#include <string>
#include <vector>

// 定义策略接口
class SortStrategy {
 public:
  virtual void sort(std::vector<int>& nums) = 0;
};

// 实现具体的策略：使用冒泡排序
class BubbleSort : public SortStrategy {
 public:
  void sort(std::vector<int>& nums) override {
    int n = nums.size();
    for (int i = 0; i < n - 1; ++i) {
      for (int j = 0; j < n - i - 1; ++j) {
        if (nums[j] > nums[j + 1]) {
          std::swap(nums[j], nums[j + 1]);
        }
      }
    }
  }
};

// 实现具体的策略：使用快速排序
class QuickSort : public SortStrategy {
 public:
  void sort(std::vector<int>& nums) override {
    quick_sort(nums, 0, nums.size() - 1);
  }

 private:
  void quick_sort(std::vector<int>& nums, int left, int right) {
    if (left < right) {
      int pivot = partition(nums, left, right);
      quick_sort(nums, left, pivot - 1);
      quick_sort(nums, pivot + 1, right);
    }
  }

  int partition(std::vector<int>& nums, int left, int right) {
    int pivot = nums[right];
    int i = left - 1;
    for (int j = left; j < right; ++j) {
      if (nums[j] < pivot) {
        ++i;
        std::swap(nums[i], nums[j]);
      }
    }
    std::swap(nums[i + 1], nums[right]);
    return i + 1;
  }
};

// 定义上下文类
class Sorter {
 public:
  void set_strategy(SortStrategy* strategy) { m_strategy = strategy; }

  void sort(std::vector<int>& nums) {
    if (m_strategy) {
      m_strategy->sort(nums);
    }
  }

 private:
  SortStrategy* m_strategy = nullptr;
};

int main() {
  Sorter sorter;

  BubbleSort bubble_sort;
  sorter.set_strategy(&bubble_sort);
  std::vector<int> nums = {3, 2, 1, 5, 4};
  sorter.sort(nums);
  for (int num : nums) {
    std::cout << num << " ";
  }
  std::cout << std::endl;

  QuickSort quick_sort;
  sorter.set_strategy(&quick_sort);
  nums = {3, 2, 1, 5, 4};
  sorter.sort(nums);
  for (int num : nums) {
    std::cout << num << " ";
  }
  std::cout << std::endl;

  return 0;
}
