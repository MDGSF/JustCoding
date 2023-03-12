#include <iostream>
#include <string>

class Product {
 public:
  virtual void operation() = 0;
};

class ProductA : public Product {
 public:
  void operation() { std::cout << "Product A operation" << std::endl; }
};

class ProductB : public Product {
 public:
  void operation() { std::cout << "Product B operation" << std::endl; }
};

class Factory {
 public:
  static Product* create_product(const std::string& type) {
    if (type == "A") {
      return new ProductA();
    } else if (type == "B") {
      return new ProductB();
    } else {
      return nullptr;
    }
  }
};

int main() {
  Product* productA = Factory::create_product("A");
  productA->operation();

  Product* productB = Factory::create_product("B");
  productB->operation();

  delete productA;
  delete productB;

  return 0;
}
