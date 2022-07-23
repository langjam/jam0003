#pragma once

#include "expr.h"

class AstMulExpr : public BinExpr {
public:
    AstMulExpr(AstExpr* lhs, AstExpr* rhs)
        : BinExpr(lhs, rhs) { }
};
