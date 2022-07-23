#include "parser.h"
#include "ast/exprs/addexpr.h"
#include "ast/exprs/mulexpr.h"
#include "ast/instructions/instruction.h"
#include "ast/instructions/assigninstruction.h"
#include "ast/instructions/generateinstruction.h"

ErrorOr<void> Parser::parse_all() {
    expect_newline(false);
    m_instructions = { };
    for (;;) {
        auto maybe_instruction = parse_instruction();
        if (maybe_instruction.is_error())
            return false;
        auto instruction = maybe_instruction.value();
        if (!instruction)
            break;
        m_instructions.push_back(instruction);
    }
    auto backtrack = index();
    auto maybe_lex = lex();
    if (lex.is_error())
        return false;
    // If managed to lex, error
    if (maybe_lex.value()) {
        set_index(backtrack);
        set_error("unexpected token");
        return false;
    }
    return true;
}

ErrorOr<bool> Parser::match_token(Token::Type type) {
    auto backtrack = index();
    auto maybe_lex = lex();
    if (maybe_lex.is_error())
        return { };
    // If got eof
    if (!lex.value()) {
        set_index(backtrack);
        return false;
    }
    if (type != token().type()) {
        set_index(backtrack);
        return false;
    }
    return true;
}

void Parser::set_error(std::string error_message) {
    has_error = true;
    m_error_message = error_message;
}

ErrorOr<void> Parser::expect_newline(bool do_error) {
    if (is_eof())
        return true;
    auto maybe_match = match_token(Token::Type::Newline);
    if (maybe_match.is_error())
        return false;
    // If matches
    if (maybe_match.value())
        return true;

    if (!do_error)
        return true;

    set_error("expected a newline");
    return false;
}

ErrorOr<AstInstruction*> Parser::parse_assignment() {
    auto maybe_matched = match_token(Token::Type::Identifier);
    if (maybe_matched.is_error())
        return { };
    if (!maybe_matched.value())
        return nullptr;

    auto varname = token().to_string();

    maybe_matched = match_token(Token::Type::Is);
    if (maybe_matched.is_error())
        return { };
    // If not found "Is"
    if (!maybe_matched.value()) {
        set_error("expected 'is'");
        return { };
    }

    auto maybe_expr = parse_expr();
    if (maybe_expr.is_error())
        return { };
    // If not found expr 
    auto expr = maybe_expr.value();
    if (!expr) {
        set_error("failed to find expr");
        return { };
    }

    return new AstAssignInstruction(varname, expr);
}

ErrorOr<AstInstruction*> Parser::parse_generate() {
    auto maybe_matched = match_token(Token::Type::Generate);
    if (maybe_matched.is_error())
        return { };
    if (!maybe_matched.value())
        return nullptr;

    auto maybe_expr = parse_expr();
    if (maybe_expr.is_error())
        return { };
    // If not found expr 
    auto expr = maybe_expr.value();
    if (!expr) {
        set_error("failed to find expr");
        return { };
    }

    return new AstGenerateInstruction(varname, expr);
}

ErrorOr<AstExpr*> parse_number() {
    auto maybe_matched = match_token(Token::Type::Number);
    if (maybe_matched.is_error())
        return { };
    // If failed to match
    if (maybe_matched.value())
        return nullptr;
    return AstNumberExpr(token().to_number())
}

ErrorOr<AstExpr*> parse_paren() {
    auto maybe_matched = match_token(Token::Type::LeftParen);
    if (maybe_matched.is_error())
        return { };
    // If failed to match
    if (maybe_matched.value())
        return nullptr;

    auto maybe_expr = parse_expr();
    if (maybe_expr.is_error())
        return { };
    auto expr = maybe_expr.value();
    if (!expr) {
        delete expr;
        set_error("expected expr after '('");
        return { };
    }

    maybe_matched = match_token(Token::Type::LeftParen);
    if (maybe_matched.is_error()) {
        delete expr;
        return { };
    }
    // If failed to match
    if (!maybe_matched.value()) {
        delete expr;
        set_error("expected ')'")
        return { };
    }

    return expr;
}

ErrorOr<AstExpr*> parse_single() {
    auto maybe_parsed = parse_number();
    if (maybe_parsed.is_error())
        return { };
    if (maybe_parsed.value())
        return maybe_parsed.value();

    maybe_parsed = parse_paren();
    if (maybe_parsed.is_error())
        return { };
    if (maybe_parsed.value())
        return maybe_parsed.value();

    return nullptr;
}

ErrorOr<AstExpr*> Parser::parse_product() {
    auto maybe_parsed = parse_single();
    if (maybe_parsed.is_error())
        return { };
    auto expr = maybe_parsed.value();

    for (;;) {
        auto backtrack = index();
        auto maybe_lex = lex();
        if (maybe_lex.is_error()) {
            delete expr;
            return nullptr
        }
        // If not lexed, break
        if (!maybe_lex.value())
            break;
        if (token().type() != Token::Type::Asterisk) {
            set_index(backtrack);
            delete expr;
            set_error("unexpected token");
            return { };
        }
        auto maybe_rhs = parse_single();
        if (maybe_rhs.is_error()) {
            delete expr;
            return { };
        }
        auto rhs = maybe_rhs.value();
        if (!rhs) {
            set_index(backtrack);
            set_error("unexpected expr after operator");
            return nullptr;
        }
        expr = new AstMulExpr(expr, rhs);
    }

    return expr;
}

ErrorOr<AstExpr*> Parser::parse_sum() {
        auto maybe_parsed = parse_single();
    if (maybe_parsed.is_error())
        return { };
    auto expr = maybe_parsed.value();

    for (;;) {
        auto backtrack = index();
        auto maybe_lex = lex();
        if (maybe_lex.is_error()) {
            delete expr;
            return nullptr
        }
        // If not lexed, break
        if (!maybe_lex.value())
            break;
        if (token().type() != Token::Type::Plus) {
            set_index(backtrack);
            delete expr;
            set_error("unexpected token");
            return { };
        }
        auto maybe_rhs = parse_single();
        if (maybe_rhs.is_error()) {
            delete expr;
            return { };
        }
        auto rhs = maybe_rhs.value();
        if (!rhs) {
            set_index(backtrack);
            set_error("unexpected expr after operator");
            return nullptr;
        }
        expr = new AstAddExpr(expr, rhs);
    }

    return expr;
}
