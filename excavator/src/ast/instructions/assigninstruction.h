#pragma once

#include <ast/exprs/expr.h>
#include <ast/instructions/instruction.h>
#include <runtime/state.h>

#include <string>

class AstAssignInstruction : public AstInstruction {
   public:
    AstAssignInstruction(std::string varname, AstExpr::Ptr expr)
        : m_varname(varname), m_value_expr(expr) {}

    void run(State& state) override {
        auto value = m_value_expr->eval(state);
        state.set_variable(m_varname, value);
    }

   private:
    std::string m_varname;
    AstExpr::Ptr m_value_expr;
};
