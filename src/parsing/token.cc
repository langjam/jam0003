#include "token.h"

#include <cassert>
#include <iostream>

std::string Token::to_string() {
    assert(type() == Identifier || type() == Number);
    return m_stream.substr(index(), index() + size());
}

int Token::to_number() {
    assert(type() == Number);
    auto as_string = to_string();
    int i = 0;
    for (auto c : as_string) {
        i *= 10;
        i += c - '0';
    }
    return i;
}
