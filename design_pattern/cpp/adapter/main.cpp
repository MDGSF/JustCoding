#include <iostream>
#include <string>

// 目标接口
class TargetInterface {
 public:
  virtual ~TargetInterface() = default;
  virtual void request() = 0;
};

// 源接口
class Adaptee {
 public:
  void specific_request() {
    std::cout << "Adaptee specific request." << std::endl;
  }
};

// 类适配器
class Adapter : public TargetInterface, private Adaptee {
 public:
  void request() override { specific_request(); }
};

// 客户端代码
void client(TargetInterface* target) { target->request(); }

int main() {
  TargetInterface* target = new Adapter();
  client(target);
  delete target;
  return 0;
}
