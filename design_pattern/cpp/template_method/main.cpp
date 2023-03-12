#include <iostream>

class AbstractClass {
 public:
  void tempalte_method() {
    primitive_operation1();
    primitive_operation2();
    concrete_operation();
  }

  virtual void primitive_operation1() = 0;
  virtual void primitive_operation2() = 0;

  void concrete_operation() {
    std::cout << "AbstractClass::concrete_operation" << std::endl;
  }
};

class ConcreteClassA : public AbstractClass {
 public:
  void primitive_operation1() {
    std::cout << "ConcreteClassA::primitive_operation1" << std::endl;
  }

  void primitive_operation2() {
    std::cout << "ConcreteClassA::primitive_operation2" << std::endl;
  }
};

class ConcreteClassB : public AbstractClass {
 public:
  void primitive_operation1() {
    std::cout << "ConcreteClassB::primitive_operation1" << std::endl;
  }

  void primitive_operation2() {
    std::cout << "ConcreteClassB::primitive_operation2" << std::endl;
  }
};

int main() {
  AbstractClass* pa = new ConcreteClassA();
  pa->tempalte_method();

  AbstractClass* pb = new ConcreteClassB();
  pb->tempalte_method();
  return 0;
}
