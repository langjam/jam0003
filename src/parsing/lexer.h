#pragma once

#include <parsing/token.h>
#include <parsing/streampos.h>
#include <utils/erroror.h>

#include <string>

class Parser;

class Lexer {
    friend Parser;

   public:
    Lexer(std::string filename, std::string& stream) : m_token(stream), m_stream(stream), m_filename(filename) {}
    ~Lexer() {}

    std::string filename() { return m_filename; }

    ErrorOr<bool> lex();

   private:
    bool m_has_error{false};
    std::string m_error_message{""};
    Token m_token;
    std::string& m_stream;
    std::string m_filename;
    StreamPos m_position { };

    bool has_error() { return m_has_error; }
    Token& token() { return m_token; }
    StreamPos position() { return m_position; }
    size_t remaining() { return m_stream.length() - m_position.index; }
    size_t is_eof() { return remaining() == 0; }
    void set_error(std::string error_message);
    void show_error();
    char get_char(size_t idx = 0);
    void advance(size_t amount = 1);
    void set_position(StreamPos position) { m_position = position; }

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
