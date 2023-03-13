#include <iostream>
#include <string>
#include <vector>

class ElementA;
class ElementB;

class Visitor {
 public:
  virtual void visit(ElementA* element) = 0;
  virtual void visit(ElementB* element) = 0;
};

class Element {
 public:
  virtual void accept(Visitor* visitor) = 0;
};

class ElementA : public Element {
 public:
  void accept(Visitor* visitor) override { visitor->visit(this); }

  void operationA() { std::cout << "Operation A of Element A" << std::endl; }
};

class ElementB : public Element {
 public:
  void accept(Visitor* visitor) override { visitor->visit(this); }

  void operationB() { std::cout << "Operation B of Element B" << std::endl; }
};

class ConcreteVisitor : public Visitor {
 public:
  void visit(ElementA* element) override { element->operationA(); }

  void visit(ElementB* element) override { element->operationB(); }
};

int main() {
  std::vector<Element*> elements{new ElementA(), new ElementB(),
                                 new ElementA()};

  ConcreteVisitor visitor;
  for (Element* element : elements) {
    element->accept(&visitor);
  }

  for (Element* element : elements) {
    delete element;
  }

  return 0;
}
