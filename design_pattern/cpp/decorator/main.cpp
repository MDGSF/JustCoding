#include <iostream>
#include <string>

// 接口定义组件的基本操作
class Component {
 public:
  virtual ~Component() = default;
  virtual void operation() = 0;
};

// 具体组件实现基本操作
class ConcreteComponent : public Component {
 public:
  void operation() override {
    std::cout << "ConcreteComponent operation." << std::endl;
  }
};

// 装饰器类包含一个指向被装饰对象的指针，并继承Component接口
class Decorator : public Component {
 public:
  explicit Decorator(Component* component) : m_component(component) {}

  virtual ~Decorator() {
    if (m_component != nullptr) {
      delete m_component;
      m_component = nullptr;
    }
  }

  void operation() override {
    if (m_component != nullptr) {
      m_component->operation();
    }
  }

 protected:
  Component* m_component;
};

// 具体装饰器A添加新的行为
class ConcreteDecoratorA : public Decorator {
 public:
  ConcreteDecoratorA(Component* component) : Decorator(component) {}

  void added_behavior() {
    std::cout << "Added behavior from ConcreteDecoratorA." << std::endl;
  }

  void operation() override {
    Decorator::operation();
    added_behavior();
  }
};

// 具体装饰器B添加新的行为
class ConcreteDecoratorB : public Decorator {
 public:
  ConcreteDecoratorB(Component* component) : Decorator(component) {}

  void added_behavior() {
    std::cout << "Added behavior from ConcreteDecoratorB." << std::endl;
  }

  void operation() override {
    Decorator::operation();
    added_behavior();
  }
};

int main() {
  Component* component = new ConcreteComponent;
  component->operation();

  component = new ConcreteDecoratorA(component);
  component->operation();

  component = new ConcreteDecoratorB(component);
  component->operation();

  delete component;

  return 0;
}
