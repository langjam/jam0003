#pragma once

#include <ast/exprs/expr.h>

class AstNumberExpr : public AstExpr {
   public:
    AstNumberExpr(int value) : AstExpr(), m_value(value) {}

    Value::Ptr eval(State& state) override;

   private:
    int m_value;

    int value() { return m_value; }
};
