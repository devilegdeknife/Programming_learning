#include <iostream>
#include <string>

std::string hello_world() { return "Hello, world!"; }

int main() {
  std::cout << hello_world() << std::endl;
  return 0;
}