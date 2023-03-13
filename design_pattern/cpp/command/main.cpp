#include <iostream>
#include <vector>

// 命令接口
class Command {
 public:
  virtual ~Command() {}
  virtual void execute() = 0;
};

// 打开文件命令
class OpenFileCommand : public Command {
 public:
  OpenFileCommand(const std::string& filename) : m_filename(filename) {}

  void execute() override {
    std::cout << "Opening file " << m_filename << std::endl;
  }

 private:
  std::string m_filename;
};

// 关闭文件命令
class CloseFileCommand : public Command {
 public:
  CloseFileCommand(const std::string& filename) : m_filename(filename) {}

  void execute() override {
    std::cout << "Closing file " << m_filename << std::endl;
  }

 private:
  std::string m_filename;
};

// 命令调用者
class Invoker {
 public:
  void set_command(Command* command) { m_command = command; }

  void execute_command() {
    if (m_command != nullptr) {
      m_command->execute();
    }
  }

 private:
  Command* m_command = nullptr;
};

int main() {
  OpenFileCommand open_file_command("example.txt");
  CloseFileCommand close_file_command("example.txt");
  Invoker invoker;

  invoker.set_command(&open_file_command);
  invoker.execute_command();

  invoker.set_command(&close_file_command);
  invoker.execute_command();
  return 0;
}
