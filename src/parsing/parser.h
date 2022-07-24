#pragma once

#include <ast/exprs/expr.h>
#include <ast/instructions/instruction.h>
#include <parsing/lexer.h>
#include <utils/erroror.h>

#include <vector>

class Parser final {
   public:
    Parser(Lexer& lexer) : m_lexer(lexer) {}
    ~Parser() {}

    ErrorOr<void> parse_all();
    void show_error();

   private:
    Lexer& m_lexer;
    bool m_has_error{false};
    std::string m_error_message{};
    std::vector<AstInstruction::Ptr> m_instructions;

    ErrorOr<bool> lex() { return m_lexer.lex(); }
    bool has_error() { return m_has_error; }
    bool is_eof() { return m_lexer.is_eof(); }
    Token& token() { return m_lexer.token(); }
    StreamPos position() { return m_lexer.position(); }
    void set_position(StreamPos position) { m_lexer.set_position(position); }
    void set_error(std::string error_message);
    ErrorOr<bool> match_token(Token::Type type);
    ErrorOr<void> expect_newline(bool do_error = true);

    ErrorOr<AstInstruction::Ptr> parse_assignment();
    ErrorOr<AstInstruction::Ptr> parse_generate();
    ErrorOr<AstInstruction::Ptr> parse_instruction();

    ErrorOr<AstExpr::Ptr> parse_number();
    ErrorOr<AstExpr::Ptr> parse_paren();
    ErrorOr<AstExpr::Ptr> parse_variable();
    ErrorOr<AstExpr::Ptr> parse_special();
    ErrorOr<AstExpr::Ptr> parse_single();
    ErrorOr<AstExpr::Ptr> parse_product();
    ErrorOr<AstExpr::Ptr> parse_sum();
    inline ErrorOr<AstExpr::Ptr> parse_expr() { return parse_sum(); }
};
