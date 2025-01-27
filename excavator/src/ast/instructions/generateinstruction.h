#pragma once

#include <ast/exprs/expr.h>
#include <ast/instructions/instruction.h>
#include <runtime/state.h>

class AstGenerateInstruction : public AstInstruction {
   public:
    AstGenerateInstruction(AstExpr::Ptr expr) : m_value_expr(expr) {}

    void run(State& state) override;

   private:
    AstExpr::Ptr m_value_expr;
};
