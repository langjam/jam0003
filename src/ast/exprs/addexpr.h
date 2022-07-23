#pragma once

#include <ast/exprs/binexpr.h>
#include <ast/exprs/expr.h>

class AstAddExpr : public BinExpr {
   public:
    AstAddExpr(AstExpr* lhs, AstExpr* rhs) : BinExpr(lhs, rhs) {}
};
