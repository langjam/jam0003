#pragma once

#include <parsing/token.h>
#include <utils/erroror.h>

#include <string>

class Parser;

class Lexer {
    friend Parser;

   public:
    Lexer(std::string& stream) : m_token(stream), m_stream(stream) {}
    ~Lexer() {}

    ErrorOr<bool> lex();

   private:
    bool m_has_error{false};
    std::string m_error_message{""};
    Token m_token;
    std::string& m_stream;
    size_t m_index{0};

    bool has_error() { return m_has_error; }
    Token& token() { return m_token; }
    size_t index() { return m_index; }
    size_t remaining() { return m_stream.length() - index(); }
    size_t is_eof() { return remaining() == 0; }
    void set_error(std::string error_message);
    void show_error();
    char get_char(size_t idx = 0);
    void advance(size_t amount = 1);
    void set_index(size_t index) { m_index = index; }

    void clear_whitespace();
    bool lex_keyword(std::string value, Token::Type token_type);
    bool lex_newline();
    bool lex_single_character();
    bool lex_number();
    bool lex_any_keyword();
    ErrorOr<bool> lex_identifier();

    static inline bool is_identifier_start_char(char c) {
        return isalpha(c) || c == '_' || c == '$';
    }
    static inline bool is_identifier_char(char c) {
        return is_identifier_start_char(c) || isdigit(c);
    }
};
