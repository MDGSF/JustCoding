#include <stdio.h>
#include <iostream>
#include <deque>

/*
1 <= n <= 100
0 < m < 50
0 < wi < 100
*/
int solution(int n, int m, int *wis) {
  std::deque<int> bag;
  int bag_max_size = n;

  for (int i = 0; i < m; ++i) {
    int wi = wis[i];
    if (wi <= bag_max_size) {
      bag_max_size += 1;
      while (!bag.empty()) {
        if (bag.front() <= bag_max_size) {
          bag_max_size += 1;
          bag.pop_front();
        } else {
          break;
        }
      }
    } else {
      bag.push_back(wi);
      for (int bagIdx = bag.size() - 2; bagIdx >= 0; bagIdx--) {
        if (bag[bagIdx] <= wi) {
          bag[bagIdx + 1] = wi;
          break;
        } else {
          bag[bagIdx + 1] = bag[bagIdx];
        }
      }

      while (bag.size() > bag_max_size) {
        bag.pop_back();
      }
    }
  }

  if (bag_max_size > 100) {
    return 100;
  }
  return bag_max_size;
}

void test_basic(
    const char *name,
    int n,
    int m,
    int *wis,
    int expected_result) {
  int result = solution(n, m, wis);
  if (result == expected_result) {
    printf("%s: success\n", name);
  } else {
    printf("%s: failed, result = %d, expected = %d\n",
        name, result, expected_result);
  }
}

void test1() {
  int wis[3] = {4, 5, 6};
  test_basic("test1", 5, 3, &wis[0], 8);
}

void test2() {
  int wis[3] = {99, 1, 1};
  test_basic("test2", 5, 3, &wis[0], 7);
}

void test3() {
  int wis[3] = {1, 1, 1};
  test_basic("test3", 1, 3, &wis[0], 4);
}

void test4() {
  int wis[] = {5, 7, 8, 9, 10, 11, 12, 13, 1};
  test_basic("test4", 5, 9, &wis[0], 13);
}

void test5() {
  int wis[] = {5, 7, 8, 9, 10, 12, 13, 11, 1};
  test_basic("test5", 5, 9, &wis[0], 13);
}

void test6() {
  int wis[] = {1};
  test_basic("test6", 100, 1, &wis[0], 100);
}

void test7() {
  int wis[] = {10, 9, 8, 7, 1};
  test_basic("test7", 1, 5, &wis[0], 2);
}

void test() {
  test1();
  test2();
  test3();
  test4();
  test5();
  test6();
  test7();
}

void submit() {
  int n = 0;
  int m = 0;
  std::cin >> n >> m;

  int wis[m];
  for (int i = 0; i < m; ++i) {
    std::cin >> wis[i];
  }

  int result = solution(n, m, wis);

  std::cout << result;
}

int main() {
  //test();
  submit();
  return 0;
}
