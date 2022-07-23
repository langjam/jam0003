#pragma once

#include "lexer.h"
#include "ast/exprs/expr.h"
#include "ast/instructions/instruction.h"
#include "utils/erroror.h"
#include <vector>

class Parser final {
public:
    Parser(Lexer& lexer)
        : m_lexer(lexer) { }
    ~Parser() { }

    ErrorOr<void> parse_all();
    void show_error();

private:
    Lexer& m_lexer;
    bool m_has_error { false };
    std::string m_error_message { };
    std::vector<AstInstruction*> m_instructions;

    ErrorOr<bool> lex() { return m_lexer.lex(); }
    bool has_error() { return m_has_error; }
    bool is_eof() { return m_lexer.is_eof(); }
    Token& token() { return m_lexer.token(); }
    size_t index() { return m_lexer.index(); }
    void set_index(size_t index) { m_lexer.set_index(index); }
    void set_error(std::string error_message);
    ErrorOr<bool> match_token(Token::Type type);
    ErrorOr<void> expect_newline(bool do_error = true);

    ErrorOr<AstInstruction*> parse_assignment();
    ErrorOr<AstInstruction*> parse_generate();
    ErrorOr<AstInstruction*> parse_instruction();

    ErrorOr<AstExpr*> parse_number();
    ErrorOr<AstExpr*> parse_paren();
    ErrorOr<AstExpr*> parse_variable();
    ErrorOr<AstExpr*> parse_single();
    ErrorOr<AstExpr*> parse_product();
    ErrorOr<AstExpr*> parse_sum();
    inline ErrorOr<AstExpr*> parse_expr() { return parse_sum(); }
};
