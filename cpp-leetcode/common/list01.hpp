#ifndef _LIST01_HPP
#define _LIST01_HPP

#include "cpplang.hpp"

struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

namespace list {

void show(const char* name, ListNode* head) {
  ListNode* cur = head;
  printf("%s: [ ", name);
  while (cur != nullptr) {
    printf("%d ", cur->val);
    cur = cur->next;
  }
  printf("]\n");
}

ListNode* from_vector(const std::vector<int>& nums) {
  ListNode* head = nullptr;
  ListNode* tail = nullptr;
  for (int i = 0; i < nums.size(); ++i) {
    int num = nums[i];
    ListNode* node = new ListNode(num);
    if (head == nullptr) {
      head = node;
      tail = node;
    } else {
      tail->next = node;
      tail = node;
    }
  }
  return head;
}

bool equal(ListNode* list1, ListNode* list2) {
  while (list1 != nullptr && list2 != nullptr) {
    if (list1->val != list2->val) {
      return false;
    }
    list1 = list1->next;
    list2 = list2->next;
  }
  return list1 == nullptr && list2 == nullptr;
}

}  // namespace list

#endif
