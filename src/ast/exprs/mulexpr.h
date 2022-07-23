#pragma once

#include <ast/exprs/binexpr.h>

class AstMulExpr : public BinExpr {
   public:
    AstMulExpr(AstExpr* lhs, AstExpr* rhs) : BinExpr(lhs, rhs) {}
};
