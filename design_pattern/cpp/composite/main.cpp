#include <iostream>
#include <list>
#include <string>

// 抽象组件类
class Component {
 public:
  virtual void add(Component* component) = 0;
  virtual void remove(Component* component) = 0;
  virtual void display() = 0;
};

// 叶子组件类
class Leaf : public Component {
 public:
  Leaf(const std::string& name) : m_name(name) {}
  void add(Component* component) override {}
  void remove(Component* component) override {}
  void display() override { std::cout << m_name << std::endl; }

 private:
  std::string m_name;
};

// 复合组件类
class Composite : public Component {
 public:
  void add(Component* component) override { m_components.push_back(component); }
  void remove(Component* component) override { m_components.remove(component); }
  void display() override {
    for (auto& component : m_components) {
      component->display();
    }
  }

 private:
  std::list<Component*> m_components;
};

int main() {
  Composite organization;

  Composite marketing;
  marketing.add(new Leaf("市场部门经理"));
  marketing.add(new Leaf("市场专员1"));
  marketing.add(new Leaf("市场专员2"));

  Composite finance;
  finance.add(new Leaf("财务部门经理"));
  finance.add(new Leaf("财务专员1"));
  finance.add(new Leaf("财务专员2"));

  Composite development;
  development.add(new Leaf("开发部门经理"));
  development.add(new Leaf("开发工程师1"));
  development.add(new Leaf("开发工程师2"));

  organization.add(&marketing);
  organization.add(&finance);
  organization.add(&development);

  std::cout << "组织架构：" << std::endl;
  organization.display();

  return 0;
}
