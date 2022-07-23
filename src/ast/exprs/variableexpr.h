#pragma once

#include <ast/exprs/expr.h>

#include <string>

class AstVariableExpr : public AstExpr {
   public:
    AstVariableExpr(std::string name) : AstExpr(), m_name(name) {}

   private:
    std::string m_name;

    std::string name() { return m_name; }
};
