#include "list01.hpp"

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
 public:
  ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
    ListNode* dumb = new ListNode(-1);
    ListNode* tail = dumb;

    while (list1 != nullptr && list2 != nullptr) {
      if (list1->val < list2->val) {
        tail->next = list1;
        list1 = list1->next;
      } else {
        tail->next = list2;
        list2 = list2->next;
      }
      tail = tail->next;
    }

    if (list1 != nullptr) {
      tail->next = list1;
    }

    if (list2 != nullptr) {
      tail->next = list2;
    }

    return dumb->next;
  }
};

void Test(const char* case_name, ListNode* list1, ListNode* list2,
          ListNode* expected_value) {
  Solution solution;
  auto result = solution.mergeTwoLists(list1, list2);
  if (list::equal(result, expected_value)) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  ListNode* list1 = list::from_vector(std::vector<int>{1, 2, 4});
  ListNode* list2 = list::from_vector(std::vector<int>{1, 3, 4});
  ListNode* expected_value =
      list::from_vector(std::vector<int>{1, 1, 2, 3, 4, 4});
  Test("Test1", list1, list2, expected_value);
}

void Test2() {
  ListNode* list1 = list::from_vector(std::vector<int>{});
  ListNode* list2 = list::from_vector(std::vector<int>{});
  ListNode* expected_value =
      list::from_vector(std::vector<int>{});
  Test("Test2", list1, list2, expected_value);
}

void Test3() {
  ListNode* list1 = list::from_vector(std::vector<int>{});
  ListNode* list2 = list::from_vector(std::vector<int>{0});
  ListNode* expected_value =
      list::from_vector(std::vector<int>{0});
  Test("Test3", list1, list2, expected_value);
}

int main() {
  Test1();
  Test2();
  Test3();
  return 0;
}
