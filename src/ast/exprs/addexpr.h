#pragma once

#include "expr.h"
#include "binexpr.h"

class AstAddExpr : public BinExpr {
public:
    AstAddExpr(AstExpr* lhs, AstExpr* rhs)
        : BinExpr(lhs, rhs) { }
};
