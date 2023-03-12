#include <iostream>
#include <string>

// 抽象产品类
class Product {
 public:
  virtual ~Product() = default;
  virtual std::string get_name() = 0;
};

// 具体产品类A
class ConcreteProductA : public Product {
 public:
  std::string get_name() override { return "ConcreteProductA"; }
};

// 具体产品类B
class ConcreteProductB : public Product {
 public:
  std::string get_name() override { return "ConcreteProductB"; }
};

// 抽象工厂类
class Factory {
 public:
  virtual ~Factory() = default;
  virtual Product* create_product() = 0;
};

// 具体工厂类A
class ConcreteFactoryA : public Factory {
 public:
  Product* create_product() override { return new ConcreteProductA(); }
};

// 具体工厂类B
class ConcreteFactoryB : public Factory {
 public:
  Product* create_product() override { return new ConcreteProductB(); }
};

int main() {
  Factory* factoryA = new ConcreteFactoryA();
  Product* productA = factoryA->create_product();
  std::cout << "Product A name: " << productA->get_name() << std::endl;
  delete factoryA;
  delete productA;

  Factory* factoryB = new ConcreteFactoryB();
  Product* productB = factoryB->create_product();
  std::cout << "Product B name: " << productB->get_name() << std::endl;
  delete factoryB;
  delete productB;

  return 0;
}
