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
    ListNode* head = nullptr;
    ListNode* tail = nullptr;

    while (list1 != nullptr && list2 != nullptr) {
      ListNode** min_list = nullptr;
      if (list1->val < list2->val) {
        min_list = &list1;
      } else {
        min_list = &list2;
      }

      if (head == nullptr) {
        head = *min_list;
        tail = *min_list;
      } else {
        tail->next = *min_list;
        tail = tail->next;
      }

      *min_list = (*min_list)->next;
    }

    if (list1 != nullptr) {
      if (head == nullptr) {
        head = list1;
        tail = list1;
      } else {
        tail->next = list1;
      }
    }

    if (list2 != nullptr) {
      if (head == nullptr) {
        head = list2;
        tail = list2;
      } else {
        tail->next = list2;
      }
    }

    return head;
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
