#include "cpplang.hpp"
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
  ListNode* mergeInBetween(ListNode* list1, int a, int b, ListNode* list2) {
    ListNode* list2_tail = tail(list2);
    ListNode* dump = new ListNode(0, list1);
    ListNode* pre = dump;
    ListNode* cur = dump->next;
    int idx = 0;
    bool need_delete_node = false;

    while (cur != nullptr) {
      if (idx == a) {
        pre->next = list2;
        need_delete_node = true;
      }

      if (idx == b) {
        list2_tail->next = cur->next;
        delete cur;
        break;
      }

      pre = cur;
      cur = cur->next;
      if (need_delete_node) {
        delete pre;
        pre = nullptr;
      }
      idx += 1;
    }

    ListNode* result = dump->next;
    delete dump;
    return result;
  }

  ListNode* tail(ListNode* head) {
    if (head == nullptr) {
      return nullptr;
    }
    while (head->next != nullptr) {
      head = head->next;
    }
    return head;
  }
};

void Test(const char* case_name, ListNode* list1, int a, int b, ListNode* list2,
          ListNode* expected_value) {
  Solution solution;
  ListNode* result = solution.mergeInBetween(list1, a, b, list2);
  list::show("result", result);
  list::show("expected_value", expected_value);
  if (list::equal(result, expected_value)) {
    printf("%s: pass\n", case_name);
  } else {
    printf("%s: failed\n", case_name);
  }
}

void Test1() {
  std::vector<int> nums1 = {0, 1, 2, 3, 4, 5};
  std::vector<int> nums2 = {1000000, 1000001, 1000002};
  ListNode* list1 = list::from_vector(nums1);
  ListNode* list2 = list::from_vector(nums2);
  int a = 3;
  int b = 4;
  ListNode* expected_value =
      list::from_vector({0, 1, 2, 1000000, 1000001, 1000002, 5});
  Test("Test1", list1, a, b, list2, expected_value);
}

void Test2() {
  std::vector<int> nums1 = {0, 1, 2, 3, 4, 5, 6};
  std::vector<int> nums2 = {1000000, 1000001, 1000002, 1000003, 1000004};
  ListNode* list1 = list::from_vector(nums1);
  ListNode* list2 = list::from_vector(nums2);
  int a = 2;
  int b = 5;
  ListNode* expected_value =
      list::from_vector({0, 1, 1000000, 1000001, 1000002, 1000003, 1000004, 6});
  Test("Test2", list1, a, b, list2, expected_value);
}

void Test3() {
  std::vector<int> nums1 = {0, 1, 2};
  std::vector<int> nums2 = {1000000, 1000001, 1000002, 1000003};
  ListNode* list1 = list::from_vector(nums1);
  ListNode* list2 = list::from_vector(nums2);
  int a = 1;
  int b = 1;
  ListNode* expected_value =
      list::from_vector({0, 1000000, 1000001, 1000002, 1000003, 2});
  Test("Test3", list1, a, b, list2, expected_value);
}

int main() {
  Test1();
  Test2();
  Test3();
  return 0;
}
