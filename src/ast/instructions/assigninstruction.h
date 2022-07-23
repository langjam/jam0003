#pragma once

#include <ast/exprs/expr.h>
#include <ast/instructions/instruction.h>
#include "ast/state.h"

#include <string>

class AstAssignInstruction : public AstInstruction {
   public:
    AstAssignInstruction(std::string varname, AstExpr* expr)
        : m_varname(varname), m_value_expr(expr) {}

    void run(State* state) {
        state->set_variable(m_varname, *m_value_expr);
    }

   private:
    std::string m_varname;
    AstExpr* m_value_expr;
};
