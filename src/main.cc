#include <fstream>
#include <sstream>

#include "lexer.h"
#include "parser.h"

auto read_file(std::string file_path) -> std::string {
    std::ifstream t(file_path);
    std::stringstream buffer;
    buffer << t.rdbuf();
    return buffer.str();
}

auto main() -> int { 
    auto chars = read_file("poop.gml");
    Lexer lex;
    lex.lex(chars);
    return 0;
}
