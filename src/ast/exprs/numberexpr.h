#pragma once

#include "expr.h"

class AstNumberExpr : public AstExpr {
public:
    AstNumberExpr(int value)
        : AstExpr(), m_value(value) { }

private:
    int m_value;

    int value() { return m_value; }
};
