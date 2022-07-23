#pragma once

#include "lexer.h"
#include "ast/exprs/expr.h"
#include "ast/instructions/instruction.h"
#include "utils/erroror.h"

class Parser final {
public:
    Parser(Lexer& lexer)
        : m_lexer(lexer) { }
    ~Parser() { }

    ErrorOr<void> parse_all();

private:
    Lexer& m_lexer;

    ErrorOr<bool> lex(Token::Type type) { return m_lexer.lex(); }
    void index(size_t index) { return m_lexer.index(); }
    void set_index(size_t index) { m_lexer.set_index(index); }
    ErrorOr<bool> match_token(Token::Type type);
    ErrorOr<void> expect_newline(bool do_error = true);

    ErrorOr<AstInstruction*> parse_assignment();
    ErrorOr<AstInstruction*> parse_generate();

    ErrorOr<AstExpr*> parse_number();
    ErrorOr<AstExpr*> parse_paren();
    ErrorOr<AstExpr*> parse_single();
    ErrorOr<AstExpr*> parse_product();
    ErrorOr<AstExpr*> parse_sum();
    inline ErrorOr<AstExpr*> parse_expr() { return parse_sum(); }
};
