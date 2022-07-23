#pragma once

#include "instruction.h"
#include "../exprs/expr.h"

class AstGenerateInstruction : public AstInstruction {
public:
    AstGenerateInstruction(AstExpr* expr)
        : m_value_expr(expr) { }

private:
    AstExpr* m_value_expr;

    AstExpr* value_expr() { return m_value_expr; }
};
