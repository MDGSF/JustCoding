#include <iostream>
#include <string>

class Request {
 public:
  Request(const std::string& type) : m_type(type) {}
  std::string get_type() { return m_type; }

 private:
  std::string m_type;
};

class Handler {
 public:
  Handler() = default;
  void set_next(Handler* next) { m_next = next; }
  virtual void handle_request(Request* request) = 0;

 protected:
  Handler* m_next = nullptr;
};

class ConcreteHandlerA : public Handler {
 public:
  void handle_request(Request* request) override {
    if (request->get_type() == "TypeA") {
      std::cout << "ConcreteHandlerA handles the request." << std::endl;
    } else if (m_next != nullptr) {
      m_next->handle_request(request);
    }
  }
};

class ConcreteHandlerB : public Handler {
 public:
  void handle_request(Request* request) override {
    if (request->get_type() == "TypeB") {
      std::cout << "ConcreteHandlerB handles the request." << std::endl;
    } else if (m_next != nullptr) {
      m_next->handle_request(request);
    }
  }
};

class ConcreteHandlerC : public Handler {
 public:
  void handle_request(Request* request) override {
    if (request->get_type() == "TypeC") {
      std::cout << "ConcreteHandlerC handles the request." << std::endl;
    } else if (m_next != nullptr) {
      m_next->handle_request(request);
    }
  }
};

int main() {
  Request* requestA = new Request("TypeA");
  Request* requestB = new Request("TypeB");
  Request* requestC = new Request("TypeC");

  Handler* handlerA = new ConcreteHandlerA();
  Handler* handlerB = new ConcreteHandlerB();
  Handler* handlerC = new ConcreteHandlerC();
  handlerA->set_next(handlerB);
  handlerB->set_next(handlerC);

  handlerA->handle_request(requestA);
  handlerA->handle_request(requestB);
  handlerA->handle_request(requestC);

  delete requestA;
  delete requestB;
  delete requestC;

  delete handlerA;
  delete handlerB;
  delete handlerC;

  return 0;
}
