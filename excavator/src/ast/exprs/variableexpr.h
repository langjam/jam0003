#pragma once

#include <ast/exprs/expr.h>

#include <string>

class AstVariableExpr : public AstExpr {
   public:
    AstVariableExpr(std::string name) : AstExpr(), m_name(name) {}

    Value::Ptr eval(State& state) override;

   private:
    std::string m_name;
};
