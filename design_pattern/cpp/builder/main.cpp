#include <iostream>
#include <memory>
#include <string>

class Calculator {
 public:
  void set_operand1(double operand1) { m_operand1 = operand1; }

  void set_operand2(double operand2) { m_operand2 = operand2; }

  void set_operator(const std::string& op) { m_operator = op; }

  double get_result() const {
    if (m_operator == "+") {
      return m_operand1 + m_operand2;
    } else if (m_operator == "-") {
      return m_operand1 - m_operand2;
    } else if (m_operator == "*") {
      return m_operand1 * m_operand2;
    } else if (m_operator == "/") {
      return m_operand1 / m_operand2;
    } else {
      throw std::runtime_error("Invalid operator");
    }
  }

 private:
  double m_operand1 = 0.0;
  double m_operand2 = 0.0;
  std::string m_operator;
};

class CalculatorBuilder {
 public:
  CalculatorBuilder() : m_calculator(new Calculator()) {}

  CalculatorBuilder& set_operand1(double operand1) {
    m_calculator->set_operand1(operand1);
    return *this;
  }

  CalculatorBuilder& set_operand2(double operand2) {
    m_calculator->set_operand2(operand2);
    return *this;
  }

  CalculatorBuilder& set_operator(const std::string& op) {
    m_calculator->set_operator(op);
    return *this;
  }

  Calculator build() const { return *m_calculator; }

 private:
  std::unique_ptr<Calculator> m_calculator;
};

int main() {
  CalculatorBuilder builder;
  Calculator calculator =
      builder.set_operand1(10.0).set_operator("+").set_operand2(5.0).build();
  std::cout << "Result: " << calculator.get_result() << std::endl;
  return 0;
}
