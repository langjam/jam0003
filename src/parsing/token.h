#pragma once

#include <cstddef>
#include <string>

class Token final {
   public:
    enum Type {
        Undefined,
        Identifier,
        Is,
        Generate,
        LeftArrow,
        RightArrow,
        Caret,
        Comma,
        Plus,
        Asterisk,
        LeftParen,
        RightParen,
        Newline,
        Number
    };

    Token(std::string& stream) : m_stream(stream) {}
    ~Token() {}

    void set_type(Type type) { m_type = type; }
    void set_index(size_t index) { m_index = index; }
    void set_size(size_t size) { m_size = size; }

    std::string to_string();
    int to_number();

    Type type() { return m_type; }
    size_t index() { return m_index; }
    size_t size() { return m_size; }

   private:
    Type m_type{Undefined};
    std::string& m_stream;
    size_t m_index{0};
    size_t m_size{0};
};
