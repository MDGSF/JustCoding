#include <iostream>
#include <string>

class Prototype {
 public:
  virtual Prototype* clone() = 0;
  virtual void print() = 0;
};

class ConcretePrototype : public Prototype {
 public:
  ConcretePrototype(const std::string& name) : m_name(name) {}

  Prototype* clone() override { return new ConcretePrototype(*this); }

  void print() { std::cout << "name: " << m_name << std::endl; }

 private:
  std::string m_name;
};

int main() {
  Prototype* p1 = new ConcretePrototype("Alice");
  Prototype* p2 = p1->clone();

  p1->print();
  p2->print();

  delete p1;
  delete p2;

  return 0;
}
