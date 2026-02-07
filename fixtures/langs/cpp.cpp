// This is a C++ comment
#include <iostream>
#include <string>

class Greeter {
private:
    std::string message;
    
public:
    Greeter(std::string msg) : message(msg) {}
    
    void greet(std::string name) {
        std::string greeting = "Hello, " + name;
        std::cout << greeting << std::endl;
    }
};

int main() {
    Greeter greeter("Welcome");
    std::string userName = "World";
    greeter.greet(userName);
    return 0;
}
