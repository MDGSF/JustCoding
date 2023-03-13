#include <iostream>
#include <string>
#include <vector>

class Colleague;
class ConcreteColleague1;
class ConcreteColleague2;
class Mediator;

class Colleague {
 public:
  virtual ~Colleague() = default;
  virtual void send(const std::string& message) = 0;
  virtual void recv(const std::string& message) = 0;
  virtual void set_mediator(Mediator* mediator) = 0;
};

class Mediator {
 public:
  virtual ~Mediator() {}
  virtual void distribute_message(Colleague* sender,
                                  const std::string& message) = 0;
};

class ConcreteColleague1 : public Colleague {
 public:
  void send(const std::string& message) override {
    std::cout << "Colleague 1 sends message: " << message << std::endl;
    m_mediator->distribute_message(this, message);
  }

  void recv(const std::string& message) override {
    std::cout << "Colleague 1 receives message: " << message << std::endl;
  }

  void set_mediator(Mediator* mediator) override { m_mediator = mediator; }

 private:
  Mediator* m_mediator = nullptr;
};

class ConcreteColleague2 : public Colleague {
 public:
  void send(const std::string& message) override {
    std::cout << "Colleague 2 sends message: " << message << std::endl;
    m_mediator->distribute_message(this, message);
  }

  void recv(const std::string& message) override {
    std::cout << "Colleague 2 receives message: " << message << std::endl;
  }

  void set_mediator(Mediator* mediator) override { m_mediator = mediator; }

 private:
  Mediator* m_mediator = nullptr;
};

class ConcreteMediator : public Mediator {
 public:
  void add_colleague(Colleague* colleague) {
    m_colleagues.push_back(colleague);
    colleague->set_mediator(this);
  }

  void distribute_message(Colleague* sender,
                          const std::string& message) override {
    for (auto colleague : m_colleagues) {
      if (colleague != sender) {
        colleague->recv(message);
      }
    }
  }

 private:
  std::vector<Colleague*> m_colleagues;
};

int main() {
  ConcreteMediator mediator;
  ConcreteColleague1 colleague1;
  ConcreteColleague2 colleague2;
  mediator.add_colleague(&colleague1);
  mediator.add_colleague(&colleague2);
  colleague1.send("Hello from colleague 1");
  colleague2.send("Hello from colleague 2");
  return 0;
}
