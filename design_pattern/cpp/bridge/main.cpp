#include <iostream>
#include <string>

class Message {
 public:
  virtual void send(const std::string& to, const std::string& message) = 0;
};

class EmailMessage : public Message {
 public:
  void send(const std::string& to, const std::string& message) override {
    std::cout << "Sending an email message to " << to << ": " << message
              << std::endl;
  }
};

class SMSMessage : public Message {
 public:
  void send(const std::string& to, const std::string& message) override {
    std::cout << "Sending an SMS message to " << to << ": " << message
              << std::endl;
  }
};

class MessageSender {
 public:
  explicit MessageSender(Message* message) { m_message = message; }
  void send_message(const std::string& to, const std::string& message) {
    m_message->send(to, message);
  }

 private:
  Message* m_message;
};

int main() {
  MessageSender* email_sender = new MessageSender(new EmailMessage());
  email_sender->send_message("john@example.com", "Hello from email!");

  MessageSender* sms_sender = new MessageSender(new SMSMessage());
  sms_sender->send_message("1234567890", "Hello from SMS!");

  return 0;
}
