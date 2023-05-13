#include <iostream>

class Node {
 public:
  Node(int val) : val(val) {}

  int val = 0;
  struct Node* next = nullptr;
};

void insert(Node* head, int after_value, int insert_value) {
  Node* cur = head;
  while (cur != nullptr) {
    if (after_value != cur->val) {
      cur = cur->next;
    } else {
      Node* new_node = new Node(insert_value);
      new_node->next = cur->next;
      cur->next = new_node;
      break;
    }
  }
}

Node* remove(Node* head, int remove_value) {
  Node* dump = new Node(0);
  dump->next = head;

  Node* pre = dump;
  Node* cur = dump->next;
  while (cur != nullptr) {
    if (cur->val != remove_value) {
      pre = cur;
      cur = cur->next;
    } else {
      pre->next = cur->next;
      delete cur;
      break;
    }
  }

  Node* new_head = dump->next;
  delete dump;
  return new_head;
}

void show(Node* head) {
  Node* cur = head;
  while (cur != nullptr) {
    std::cout << cur->val << " ";
    cur = cur->next;
  }
  std::cout << std::endl;
}

int main() {
  int node_num;
  std::cin >> node_num;
  int head_value;
  std::cin >> head_value;

  Node* head = new Node(head_value);

  for (int i = 1; i < node_num; ++i) {
    int after_value;
    int insert_value;
    std::cin >> insert_value >> after_value;
    insert(head, after_value, insert_value);
  }

  int remove_value;
  std::cin >> remove_value;
  Node* new_head = remove(head, remove_value);

  show(new_head);

  return 0;
}
