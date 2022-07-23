#include "lexer.h"
#include <cassert>

void Lexer::set_error(std::string error_message) {
    has_error = true;
    m_error_message = error_message;
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
    token()->set_type(Token::Type::Newline);
    token()->set_index(index);
    token()->set_size(size);
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
    auto start_index = index();
    if (get_char() != '$')
        return false;
    advance();
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
    for (;;) {
        if (is_eof())
            break;
        if (!is_identifier_char(get_char()))
            break;
        advance();
    }
    token().set_type(Token::Type::Identifier);
    token().set_index(start_index + 1);
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
        if (is_identifier_char(get_char(value.length())))
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
    if (lex_identifier())
        return true;
    if (lex_any_keyword())
        return true;
    set_error("failed to lex");
    return { };
}
