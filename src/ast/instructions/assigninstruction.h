#pragma once

#include "instruction.h"

class AstAssignInstruction : public AstInstruction {
public:
    AstAssignInstruction(std::string varname, AstExpr* expr)
        : m_varname(varname), m_expr(expr) { }

private:
    std::string m_varname;
    AstExpr* m_value_expr;
};
