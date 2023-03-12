#include <iostream>
#include <string>

// 子系统A
class SubsystemA {
 public:
  void operationA() { std::cout << "Subsystem A operation" << std::endl; }
};

// 子系统B
class SubsystemB {
 public:
  void operationB() { std::cout << "Subsystem B operation" << std::endl; }
};

// 外观
class Facade {
 public:
  Facade() {
    m_subsystemA = new SubsystemA();
    m_subsystemB = new SubsystemB();
  }

  ~Facade() {
    delete m_subsystemA;
    delete m_subsystemB;
  }

  void operation() {
    m_subsystemA->operationA();
    m_subsystemB->operationB();
  }

 private:
  SubsystemA* m_subsystemA;
  SubsystemB* m_subsystemB;
};

int main() {
  Facade facade;
  facade.operation();
  return 0;
}
