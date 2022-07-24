#pragma once

#include <ast/exprs/expr.h>
#include "parsing/token.h"

class AstKeywordExpr : public AstExpr {
   public:
    AstKeywordExpr(Token::Type keyword) : m_keyword(keyword) {}

   private:
    Token::Type m_keyword;
};
