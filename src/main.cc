#include <parsing/lexer.h>
#include <parsing/parser.h>
#include <runtime/interpreter.h>

#include <fstream>
#include <iostream>
#include <sstream>

static auto read_file(std::string file_path, std::string& out) -> bool {
    std::ifstream stream(file_path);
    if (!stream.is_open()) return false;
    std::stringstream buffer;
    buffer << stream.rdbuf();
    out = buffer.str();
    return true;
}

auto main(int argc, char* argv[]) -> int {
    if (argc < 2) {
        std::cerr << "Expected filename as the first command line argument"
                  << std::endl;
        return 1;
    }
    std::string char_stream;
    if (!read_file(argv[1], char_stream)) {
        std::cerr << "Unable to open file" << std::endl;
        return 1;
    }
    Lexer lexer(argv[1], char_stream);
    Parser parser(lexer);
    if (parser.parse_all().is_error()) {
        parser.show_error();
        return 1;
    }
    Interpreter interpreter(parser.instructions());
    interpreter.run_code();
    interpreter.run_simulation();
    return 0;
}
