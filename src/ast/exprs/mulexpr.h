#pragma once

#include <ast/exprs/binexpr.h>

class AstMulExpr : public BinExpr {
   public:
    AstMulExpr(AstExpr::Ptr lhs, AstExpr::Ptr rhs) : BinExpr(lhs, rhs) {}
};
