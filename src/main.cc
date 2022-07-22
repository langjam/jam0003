#include <fstream>
#include <sstream>

auto read_file(std::string file_path) -> std::string {
  std::ifstream t(file_path);
  std::stringstream buffer;
  buffer << t.rdbuf();
  return buffer.str();
}

auto main() -> int {
    return 0;
}
