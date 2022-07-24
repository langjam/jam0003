#pragma once

#include <ast/exprs/expr.h>
#include <ast/instructions/instruction.h>
#include "ast/state.h"

class AstGenerateInstruction : public AstInstruction {
   public:
    AstGenerateInstruction(AstExpr::Ptr expr) : m_value_expr(expr) {}

    void run(State* state) {
        return;
    }

   private:
    AstExpr::Ptr m_value_expr;

    AstExpr::Ptr value_expr() { return m_value_expr; }
};
