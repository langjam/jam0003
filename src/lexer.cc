#include "lexer.h"
#include <cassert>
#include <iostream>

void Lexer::set_error(std::string error_message) {
    m_has_error = true;
    m_error_message = error_message;
}

void Lexer::show_error() {
    assert(m_has_error);
    std::cerr << "LexerError: " << m_error_message << std::endl;
}

char Lexer::get_char(size_t idx) {
    auto actual_index = index() + idx;
    assert(actual_index < m_stream.length());
    return m_stream[actual_index];
}

void Lexer::advance(size_t amount) {
    auto new_index = index() + amount;
    assert(new_index < m_stream.length());
    m_index = new_index;
}

void Lexer::clear_whitespace() {
    for (;;) {
        if (is_eof())
            break;
        bool found_whitespace;
        switch (get_char()) {
        case '\t':
        case ' ':
            found_whitespace = true;
            advance();
            break;
        default:
            found_whitespace = false;
        }
        if (!found_whitespace)
            break;
    }
}

bool Lexer::lex_newline() {
    auto start_index = index();
    if (get_char() != '\n')
        return false;
    size_t size = 0;
    do {
        advance();
        ++size;
        clear_whitespace();
    } while (!is_eof() && get_char() == '\n');
    token().set_type(Token::Type::Newline);
    token().set_index(start_index);
    token().set_size(size);
    return true;
}

bool Lexer::lex_number() {
    if (!isdigit(get_char()))
        return false;
    auto start_index = index();
    size_t length = 0;
    do {
        advance();
        ++length;
    } while (!is_eof() && isdigit(get_char()));
    token().set_type(Token::Type::Number);
    token().set_index(start_index);
    token().set_size(length);
    return true;
}

bool Lexer::lex_single_character() {
    auto start_index = index();
    Token::Type token_type;
    switch (get_char()) {
    case '<':
        token_type = Token::Type::LeftArrow;
        break;
    case '>':
        token_type = Token::Type::RightArrow;
        break;
    case '^':
        token_type = Token::Type::Caret;
        break;
    case ',':
        token_type = Token::Type::Comma;
        break;
    case '+':
        token_type = Token::Type::Plus;
        break;
    case '*':
        token_type = Token::Type::Asterisk;
        break;
    case '(':
        token_type = Token::Type::LeftParen;
        break;
    case ')':
        token_type = Token::Type::RightParen;
        break;
    default:
        return false;
    }

    advance();
    token().set_type(token_type);
    token().set_index(start_index);
    token().set_size(1);
    return true;
}

ErrorOr<bool> Lexer::lex_identifier() {
    if (get_char() != '$')
        return false;
    advance();
    auto start_index = index();
    if (is_eof()) {
        set_index(start_index);
        set_error("expected character");
        return { };
    }
    if (!is_identifier_start_char(get_char())) {
        set_index(start_index);
        set_error("unexpected value");
        return { };
    }
    advance();
    size_t identifier_length = 1;
    while (!is_eof() && is_identifier_char(get_char())) {
        advance();
        ++identifier_length;
    }
    token().set_type(Token::Type::Identifier);
    token().set_index(start_index);
    token().set_size(identifier_length);
    return true;
}

bool Lexer::lex_keyword(std::string value, Token::Type token_type) {
    if (remaining() < value.length())
        return false;
    auto start_index = index();
    for (size_t i = 0; i < value.length(); ++i) {
        if (tolower(get_char(i)) != tolower(value[i]))
            return false;
    }
    if (remaining() > value.length()) {
        // A keyword cannot end with a digit, character or any similar character
        if (is_identifier_char(get_char(value.length() + 1)))
            return false;
    }
    advance(value.length());
    token().set_type(token_type);
    token().set_index(start_index);
    token().set_size(value.length());
    return true;
}

bool Lexer::lex_any_keyword() {
    if (lex_keyword("is", Token::Type::Is))
        return true;
    if (lex_keyword("generate", Token::Type::Generate))
        return true;
    return false;
}

ErrorOr<bool> Lexer::lex() {
    clear_whitespace();
    if (is_eof())
        return false;
    if (lex_newline())
        return true;
    if (lex_single_character())
        return true;
    if (lex_any_keyword())
        return true;
    if (lex_number())
        return true;
    auto maybe_lex = lex_identifier();
    if (maybe_lex.is_error())
        return { };
    if (maybe_lex.value())
        return true;
    set_error("failed to lex");
    return { };
}
