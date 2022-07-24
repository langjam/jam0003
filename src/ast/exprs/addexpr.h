#pragma once

#include <ast/exprs/binexpr.h>
#include <ast/exprs/expr.h>

class AstAddExpr : public BinExpr {
   public:
    AstAddExpr(AstExpr::Ptr lhs, AstExpr::Ptr rhs) : BinExpr(lhs, rhs) {}

    Value::Ptr eval(State& state) override;
};
