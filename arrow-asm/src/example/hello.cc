#include "hello_world.hh"

#include "spdlog/spdlog.h"
#include <iostream>

int main() {
  SPDLOG_WARN("hello");
  std::cout << hello_world();
  return 0;
}
