#pragma once

#include "instruction.h"

class AstAssignInstruction : public AstInstruction {
public:
    AstAssignInstruction(AstExpr* expr)
        : m_expr(expr) { }

private:
    AstExpr* m_value_expr;
};
