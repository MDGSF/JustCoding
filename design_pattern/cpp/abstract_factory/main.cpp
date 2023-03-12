#include <iostream>
#include <string>

// 抽象产品A
class AbstractProductA {
 public:
  virtual void operationA() = 0;
};

// 抽象产品B
class AbstractProductB {
 public:
  virtual void operationB() = 0;
};

// 具体产品A1
class ProductA1 : public AbstractProductA {
 public:
  void operationA() override {
    std::cout << "Product A1 operation" << std::endl;
  }
};

// 具体产品A2
class ProductA2 : public AbstractProductA {
 public:
  void operationA() override {
    std::cout << "Product A2 operation" << std::endl;
  }
};

// 具体产品B1
class ProductB1 : public AbstractProductB {
 public:
  void operationB() override {
    std::cout << "Product B1 operation" << std::endl;
  }
};

// 具体产品B2
class ProductB2 : public AbstractProductB {
 public:
  void operationB() override {
    std::cout << "Product B2 operation" << std::endl;
  }
};

// 抽象工厂
class AbstractFactory {
 public:
  virtual AbstractProductA* create_product_A() = 0;
  virtual AbstractProductB* create_product_B() = 0;
};

// 具体工厂1
class ConcreteFactory1 : public AbstractFactory {
 public:
  AbstractProductA* create_product_A() override { return new ProductA1(); }
  AbstractProductB* create_product_B() override { return new ProductB1(); }
};

// 具体工厂2
class ConcreteFactory2 : public AbstractFactory {
 public:
  AbstractProductA* create_product_A() override { return new ProductA2(); }
  AbstractProductB* create_product_B() override { return new ProductB2(); }
};

int main() {
  AbstractFactory* factory1 = new ConcreteFactory1();
  AbstractProductA* productA1 = factory1->create_product_A();
  AbstractProductB* productB1 = factory1->create_product_B();
  productA1->operationA();
  productB1->operationB();

  AbstractFactory* factory2 = new ConcreteFactory2();
  AbstractProductA* productA2 = factory2->create_product_A();
  AbstractProductB* productB2 = factory2->create_product_B();
  productA2->operationA();
  productB2->operationB();

  return 0;
}
